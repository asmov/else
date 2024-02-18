use crate::error::*;
use crate::Identifiable;
use crate::World;
use crate::identity::{UID, ClassID};

/// Performs all write operations for game data objects. Nothing is mutated directly on the object itself.  
/// Respective to its `BuilderMode` construction, initialization and finalization is handled by:
/// - BuilderMode::Creator => creator() and create()
/// - BuilderMode::Editor  => editor() and modify()
pub trait Builder: Sized {
    /// The model struct that this builder ultimately creates. If the model is a variant of an enum (like Thing), then
    /// BuilderType is that enum instead.
    type ModelType: Sized;
    /// The builder struct that is returned on creation and modification. Typically, Self, unless we're a variant of
    /// of a Builder enum (like ThingBuilder). In which case, typically, the BuilderType is that enum instead.
    type BuilderType: Builder;

    fn creator() -> Self;

    fn editor() -> Self;

    fn builder(mode: BuilderMode) -> Self {
        match mode {
            BuilderMode::Creator => Self::creator(),
            BuilderMode::Editor => Self::editor()
        }
    }

    fn builder_mode(&self) -> BuilderMode;

    fn create(self) -> Result<Creation<Self::BuilderType>>; 

    fn modify(self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>>; 

    fn set(&mut self, _raw_field: &str, _raw_value: String) -> Result<()> {
        unimplemented!("Builder::set()")
    }

    fn try_assign_value<T: Clone>(builder_option: &mut Option<T>, field: impl Fields) -> Result<T> {
        let field = field.field();
        let value = builder_option
            .as_ref()
            .ok_or_else(|| Error::FieldNotSet {class: field.classname(), field: field.name()})?
            .clone();

        Ok(value)
    }

    fn sync_modify(self, _world: &mut World) -> Result<Modification<Self::BuilderType>> {
        unimplemented!("Builder::sync_modify()")
    }

    fn class_id(&self) -> ClassID;
}


/// Provides the static creator() and editor() methods for a data type.
pub trait Built {
    type BuilderType: Builder;

    fn creator() -> Self::BuilderType {
        Self::BuilderType::creator()
    }

    fn editor() -> Self::BuilderType {
        Self::BuilderType::editor()
    }

    fn builder(mode: BuilderMode) -> Self::BuilderType {
        Self::BuilderType::builder(mode)
    }
}

/// Determines wheter a new data object is being created or an existing one is being modified.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BuilderMode {
    Creator,
    Editor
}

/// The result of a Builder::create() call. It is what is serialized and sync'd out to any mirrors, if necessary.
///
/// Implementation requires that a Builder and its BuilderType are the same. Thus, when using an enum dispatch pattern,
/// the variant's Builder::BuilderType should be the enum's Builder (not Self).
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Creation<B>
where
    B: Builder
{
    builder: B,
    model: B::ModelType   
}

impl<B> Creation<B>
where
    B: Builder<BuilderType = B>
{
    pub fn new(builder: B, model: B::ModelType) -> Self {
        Self {
            builder,
            model
        }
    }

    pub fn builder(&self) -> &B {
        &self.builder
    }

    pub fn model(&self) -> &B::ModelType {
        &self.model
    }

    pub fn split_option(self) -> (Option<B>, B::ModelType) {
        (Some(self.builder), self.model)
    }

    pub fn split(self) -> (B, B::ModelType) {
        (self.builder, self.model)
    }

    pub fn try_assign(creator: &mut Option<B>, field: impl Fields) -> Result<B::ModelType> {
        let field = field.field();
        let (builder, model)= creator.take()
            .ok_or_else(|| Error::FieldNotSet {class: field.classname(), field: field.name()})?
            .create()?
            .split();

        let _ = creator.insert(builder);
        Ok(model)
    }

    pub fn assign(creator_option: &mut Option<B>) -> Result<B::ModelType> {
        let (builder, model) = creator_option.take().unwrap()
            .create()?
            .split();
        let _ = creator_option.insert(builder);
        Ok(model)
    }

    pub fn assign_vec(creators: &mut Vec<B>) -> Result<Vec<B::ModelType>> {
        Ok(creators
            .drain(0..)
            .map(|creator| creator.create())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|creation| {
                let (builder, model) = creation.split();
                creators.push(builder);
                model
            })
            .collect())
    }

    pub fn assign_vec_uid(creators: &mut Vec<B>) -> Result<Vec<UID>>
    where
        B::ModelType: Identifiable
    {
        Ok(creators
            .drain(0..)
            .map(|creator| creator.create())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|creation| {
                let (builder, model) = creation.split();
                creators.push(builder);
                model.uid()
            })
            .collect())
    }


    pub fn modify_vec(creators: &mut Vec<B>, originals: &mut Vec<B::ModelType>) -> Result<()> {
       Ok(creators 
            .drain(0..)
            .map(|creator| creator.create())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .for_each(|creation| {
                let (builder, model) = creation.split();
                originals.push(model);
                creators.push(builder);
            }))
    }

    pub fn modify_vec_uid(creators: &mut Vec<B>, originals: &mut Vec<UID>) -> Result<()>
    where
        B::ModelType: Identifiable
    {
       Ok(creators 
            .drain(0..)
            .map(|creator| creator.create())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .for_each(|creation| {
                let (builder, model) = creation.split();
                originals.push(model.uid());
                creators.push(builder);
            }))
    }
}

/// The result of a Builder::modify() call. It is what is serialized and sync'd out to any mirrors, if necessary.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Modification<B: Builder> {
    #[serde(skip)]
    fields_changed: Vec<&'static Field>,
    builder: B
}

impl<B> Modification<B>
where
    B: Builder<BuilderType = B>
{
    pub fn new(builder: B, fields_changed: Vec<&'static Field>) -> Self {
        Self {
            fields_changed,
            builder
        }
    }

    pub fn fields_changed(&self) -> &Vec<&'static Field> {
        &self.fields_changed
    }

    pub fn builder(&self) -> &B {
        &self.builder
    }

    pub fn take_builder(self) -> B {
        self.builder
    }

    pub fn assign(editor_option: &mut Option<B>, original_value: &mut B::ModelType) -> Result<()> {
        let builder = editor_option.take().unwrap()
            .modify(original_value)?
            .take_builder();
        let _ = editor_option.insert(builder);
        Ok(())
    }
}

pub trait Fields {
    fn field(&self) -> &'static Field;
}

pub trait Class: Fields {
    fn class_id() -> ClassID;
    fn classname() -> &'static str;
}

/// Represents data types for model fields that are available to APIs.
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum FieldValueType {
    /// bool
    Bool,
    /// i64
    I64,
    /// u64
    U64,
    /// f64
    F64,
    /// u128
    UID,
    /// String
    String, 
    /// Fieldless enum
    Enum,
    /// impl Builder
    Model,
    /// Vec<UID>
    VecUID,
    /// Vec<impl Builder>
    VecModel,
    /// Vec<String>
    VecString,
}

/// Represents a specific field of a model that is available to APIs
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Field {
    class_id: ClassID,
    classname: &'static str,
    name: &'static str,
    value_type: FieldValueType
}

impl Field {
    pub const fn new(class_id: ClassID, classname: &'static str, name: &'static str, value_type: FieldValueType) -> Self {
        Self {
            class_id,
            classname,
            name,
            value_type
        }
    }

    pub const fn class_id(&self) -> ClassID {
        self.class_id
    }

    pub const fn classname(&self) -> &'static str {
        self.classname
    }

    pub const fn name(&self) -> &'static str {
        &self.name
    }

    pub const fn value_type(&self) -> FieldValueType {
        self.value_type
    }
}