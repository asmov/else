use crate::error::*;

/// Performs all write operations for game data objects. Nothing is mutated directly on the object itself.  
/// Respective to its `BuilderMode` construction, initialization and finalization is handled by:
/// - BuilderMode::Creator => creator() and create()
/// - BuilderMode::Editor  => editor() and modify()
pub trait Builder: Sized {
    /// The model struct that this builder ultimately creates. If the model is a variant of an enum (like Thing), then
    /// BuilderType is that enum instead.
    type Type;
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

    fn create(self) -> Result<Self::Type>; 

    fn modify(self, original: &mut Self::Type) -> Result<Modification<Self::BuilderType>>; 

    fn set(&mut self, raw_field: &str, raw_value: String) -> Result<()> {
        todo!()
    }
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
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Creation<B: Builder> {
    builder: B
}

impl<B: Builder> Creation<B> {
    pub fn new(builder: B) -> Self {
        Self {
            builder
        }
    }

    pub fn builder(&self) -> &B {
        &self.builder
    }
}

/// The result of a Builder::modify() call. It is what is serialized and sync'd out to any mirrors, if necessary.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Modification<B: Builder> {
    #[serde(skip)]
    fields_changed: Vec<&'static Field>,
    builder: B
}

impl<B: Builder> Modification<B> {
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
}


/// Represents data types for model fields that are available to APIs.
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum FieldValueType {
    /// String
    String, 
    /// i64
    Integer,
    /// u64
    UnsignedInteger,
    /// f64
    Float,
    /// bool
    Boolean,
    Object,
    /// Vec<Identity>
    ObjectIDArray,
    ObjectArray,
    /// Vec<String>
    StringArray,
    /// Fieldless enum
    Enum
}

/// Represents a specific field of a model that is available to APIs
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Field {
    name: &'static str,
    value_type: FieldValueType
}

impl Field {
    pub const fn new(name: &'static str, value_type: FieldValueType) -> Self {
        Self {
            name,
            value_type
        }
    }

    pub const fn name(&self) -> &'static str {
        &self.name
    }

    pub const fn value_type(&self) -> FieldValueType {
        self.value_type
    }
}