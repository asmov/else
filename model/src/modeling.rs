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

use std::{fmt::Display, str::FromStr};

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

    pub fn split(self) -> (B, B::ModelType) {
        (self.builder, self.model)
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

/// Represents a struct that is not a model and does not have a builder.
/// It must be able to be constucted by a string.
/// The results of FromStr and Display should be interchangeable.
pub trait NonPrimitive: FromStr + Display + Sized {
    fn class_ident(&self) -> &'static ClassIdent;
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
   /// String
    String, 
    /// u128
    UID(&'static ClassIdent),
     /// Fieldless enum
    Enum(&'static ClassIdent),
    /// No builder, implements NonPrimitive (FromStr, Display, Sized)
    NonPrimitive(&'static ClassIdent),
    /// impl Builder
    Model(&'static ClassIdent),
    /// Vec<UID>
    UIDList(&'static ClassIdent),
    /// Vec<impl Builder>
    ModelList(&'static ClassIdent),
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