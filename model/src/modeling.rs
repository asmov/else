//! # Else Modeling
//! This module defines the core system and standards that all else models are built upon and adhere to. Each model
//! defines an accompanying [Builder] and schema ([Class], [Fields]).
//! ## Builder
//! A builder is used to create and modify its model type. Direct mutation of a model object is performed solely through
//! [Builder::modify()].
//! ## Schema
//! A model's schema definition is used by its Builder and modeling helpers. Schema definitions are defined as an `enum`
//! that implements [Fields] and [Class]. Each variant represents a field of its model.
//! ## Creation and Modification
//! The result of a [Builder::create()] or [Builder::modify()] is a [Creation] or [Modification] object, respectively.
//! They act as a containers for both a fully processed Builder and a [FieldsChanged] report. Creation and Modification
//! objects can be serialized and transmitted, allowing for incremental synchronization of state. FieldsChanged is not
//! synchronized, however, as it reports solely on changes to the local system's state.
//! Refer to [Sync] for more information.

pub mod fields_changed;
pub mod build;

use crate::{error::*, identity::*};

pub use fields_changed::*;
pub use build::*;

/// Performs all construction and mutation operations for its corresponding model type.
///
/// It operates in one of two modes at a time: [BuilderMode::Creator] or [BuilderMode::Editor]. Creator constructs and Editor mutates. The [BuilderMode]
/// is set at initialization, typically using [Builder::creator()] or [Builder::editor()].
///
/// Setters are provided for each field of a the corresponding model. Internally, in a Builder's struct, each setter stores an operation using either
/// `Option` for a single value or or [ListOp] for multiple values. Mutation occurs only against what is set.
///
/// Ultimately:
/// - Creator uses [Builder::create()] to construct a new model, returning a [Creation].
/// - Editor uses [Builder::modify()] to mutate an existing model, returning a [Modification].
///
/// Refer to the module documentation for more information.
pub trait Builder: Sized {
    /// The model that stores all state for the rest of its suite. This is expected to be some form of state container
    /// for the rest of the models, all of which share the same DomainType.
    type DomainType: Sized;

    /// The builder struct that is returned on creation and modification. Typically, Self, unless we're a variant of
    /// of a Builder enum (like ThingBuilder). In which case, typically, the BuilderType is that enum instead.
    type BuilderType: Builder;

    /// The model struct that this builder ultimately creates. If the model is a variant of an enum (like Thing), then
    /// BuilderType is that enum instead.
    type ModelType: Sized;

    fn creator() -> Self;

    fn editor() -> Self;

    fn builder_mode(&self) -> BuilderMode;

    fn class_ident(&self) -> &'static ClassIdent;

    fn create(self) -> Result<Creation<Self::BuilderType>>; 

    fn modify(self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>>; 

    fn set(&mut self, _raw_field: &str, _raw_value: String) -> Result<()> {
        unimplemented!("Builder::set()")
    }

    /// Typically called by [Sync] to systematically synchronize state changes with an upstream provider.
    fn synchronize(self, _domain: &mut Self::DomainType) -> Result<Modification<Self::BuilderType>> {
        unimplemented!("Builder::modify_domain()")
    }

    fn builder(mode: BuilderMode) -> Self {
        match mode {
            BuilderMode::Creator => Self::creator(),
            BuilderMode::Editor => Self::editor()
        }
    }
}

/// Determines wheter a new data object is being created or an existing one is being modified.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BuilderMode {
    Creator,
    Editor
}

/// Provides the static creator() and editor() methods for a model.
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

    fn edit_self(&self) -> Self::BuilderType
    where
        Self: Identifiable,
        Self::BuilderType: BuildableIdentity
    {
        let mut editor = Self::editor();
        let identity_builder = IdentityBuilder::editor_from_uid(self.uid());
        editor.identity(identity_builder).unwrap();
        editor
    }
}

/// Represents an Add, Remove, or Modify operation against a Vec
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ListOp<T: MaybeIdentifiable, R: MaybeIdentifiable> {
    Add(T),
    Edit(T),
    Remove(R)
}

impl<T: MaybeIdentifiable, R: MaybeIdentifiable> ListOp<T, R> {
    pub fn is_add(&self) -> bool {
        match self {
            ListOp::Add(_) => true,
            _ => false,
        }
    }

    pub fn is_edit(&self) -> bool {
        match self {
            ListOp::Edit(_) => true,
            _ => false
        }
    }

    pub fn is_remove(&self) -> bool {
        match self {
            ListOp::Remove(_) => true,
            _ => false
        }
    }
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


    //todo: conditionally call create() or modify() based on the builder's mode
    //todo: move this to the Builder trait
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
    builder: B,
    #[serde(skip)]
    fields_changed: FieldsChanged
}

impl<B> Modification<B>
where
    B: Builder<BuilderType = B>
{
    pub fn new_old(builder: B, _fields_changed_old: Vec<&'static Field>) -> Self {
        Self {
            fields_changed: FieldsChanged::new(builder.class_ident(), ChangeOp::Modify),
            builder,
        }
    }

    pub fn new(builder: B, fields_changed: FieldsChanged) -> Self {
        Self {
            builder,
            fields_changed,
        }
    }

    pub fn builder(&self) -> &B {
        &self.builder
    }

    pub fn fields_changed(&self) -> &FieldsChanged {
        &self.fields_changed
    }

    pub fn split(self) -> (B, FieldsChanged) {
        (self.builder, self.fields_changed)
    }

    pub fn take_builder(self) -> B {
        self.builder
    }
}

pub trait Fields {
    fn field(&self) -> &'static Field;
}

pub trait Class: Fields {
    fn class_ident() -> &'static ClassIdent;
    fn class_id() -> ClassID { Self::class_ident().class_id() }
    fn classname() -> &'static str { Self::class_ident().classname() }
}

/// Represents data types for model fields that are available to APIs.
#[derive(Clone, Copy, Debug)]
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
    Model(&'static ClassIdent),
    /// Vec<UID>
    UIDList,
    /// Vec<impl Builder>
    ModelList,
    /// Vec<String>
    StringList,
}

#[derive(Debug)]
pub struct ClassIdent {
    class_id: ClassID,
    classname: &'static str
}

impl ClassIdent {
    pub const fn new(class_id: ClassID, classname: &'static str) -> Self {
        Self {
            class_id,
            classname
        }
    }

    pub const fn class_id(&self) -> ClassID {
        self.class_id
    }

    pub const fn classname(&self) -> &'static str {
        self.classname
    }
}

/// Represents a specific field of a model that is available to APIs
#[derive(Debug)]
pub struct Field {
    class_ident: &'static ClassIdent,
    name: &'static str,
    value_type: FieldValueType
}

impl Field {
    pub const fn new(class_ident: &'static ClassIdent, name: &'static str, value_type: FieldValueType) -> Self {
        Self {
            class_ident,
            name,
            value_type
        }
    }

    pub const fn class_id(&self) -> ClassID {
        self.class_ident.class_id()
    }

    pub const fn classname(&self) -> &'static str {
        self.class_ident.classname()
    }

    pub const fn class_ident(&self) -> &'static ClassIdent {
        self.class_ident
    }

    pub const fn name(&self) -> &'static str {
        &self.name
    }

    pub const fn value_type(&self) -> FieldValueType {
        self.value_type
    }
}