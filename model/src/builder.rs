use crate::error::*;

pub trait Builder: Sized + serde::Serialize + serde::de::DeserializeOwned {
    type Type;

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

    fn modify(self, original: &mut Self::Type) -> Result<Modification<Self>>; 

    fn set(&mut self, raw_field: &str, raw_value: String) -> Result<()> {
        todo!()
    }
}

/// Provides the static creator() and editor() methods.
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BuilderMode {
    Creator,
    Editor
}

#[derive(Debug)]
pub struct Modification<B: Builder> {
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

    pub fn take_builder(self) -> B {
        self.builder
    }
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Debug)]
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