use crate::{s, error::*, builder::*};
use serde;

/// All descriptive information about and object that can be observed by a player.
/// See also its corresponding trait: `Descriptive`
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Descriptor {
    /// The title
    name: String,
    /// Any term that might be used to reference this
    keywords: Vec<String>,
    /// Unique to the World. Should be used to permanently reference objects (never use ID).
    key: Option<String>,
    /// A one-liner summary. If `description` is not available, this should be used instead.
    short_description: Option<String>,
    /// A detailed and narrative description.
    description: Option<String>,
    /// Development notes from authors and editors. Not seen during normal play.
    notes: Option<String>
}

/* 
impl bincode::Encode for Descriptor {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> std::prelude::v1::Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.name, encoder)?;
        Ok(())
    }
}

impl bincode::Decode for Descriptor {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> std::prelude::v1::Result<Self, bincode::error::DecodeError> {
        Ok(Self{
            name: bincode::Decode::decode(decoder)?,
            keywords: bincode::Decode::decode(decoder)?,
            key: None,//bincode::Decode::decode(decoder)?,
            short_description: bincode::Decode::decode(decoder)?,
            description: bincode::Decode::decode(decoder)?,
            notes: bincode::Decode::decode(decoder)?
        })
    }
}

impl<'de> bincode::BorrowDecode<'de> for Descriptor {
    fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(decoder: &mut D) -> std::prelude::v1::Result<Self, bincode::error::DecodeError> {
        Ok(Self{
            name: bincode::BorrowDecode::borrow_decode(decoder)?,
            keywords: bincode::BorrowDecode::borrow_decode(decoder)?,
            key: bincode::BorrowDecode::borrow_decode(decoder)?,
            short_description: bincode::BorrowDecode::borrow_decode(decoder)?,
            description: bincode::BorrowDecode::borrow_decode(decoder)?,
            notes: bincode::BorrowDecode::borrow_decode(decoder)?
        })
    }
}
*/


/// The trait that provides standard immutable access to a `Descriptor` struct
pub trait Descriptive {
    /// Fetch the `Descriptor` struct for this object
    fn descriptor(&self) -> &Descriptor;

    /// The title
    fn name(&self) -> &str {
        &self.descriptor().name
    }

    /// Any term that might be used to reference this
    fn keywords(&self) -> &Vec<String> {
        &self.descriptor().keywords
    }

    /// Unique to the World. Should be used to permanently reference objects (never use ID).
    fn key(&self) -> Option<&String> {
        self.descriptor().key.as_ref()
    }

    /// A one-liner summary. If `description` is not available, this will be used instead.
    fn short_description(&self) -> Option<&String> {
        self.descriptor().short_description.as_ref()
    }

    /// A detailed and narrative description. If this doesn't exist, `short_description` will be used instead. 
    fn description(&self) -> Option<&String> {
        self.descriptor().description.as_ref()
            .or_else(|| self.short_description())
    }

    /// Development notes from authors and editors. Not seen during normal play.
    fn notes(&self) -> Option<&String> {
        self.descriptor().notes.as_ref()
    }
}

pub trait DescriptiveMut: Descriptive {
    fn descriptor_mut(&mut self) -> &mut Descriptor;
}

impl Descriptive for Descriptor {
    fn descriptor(&self) -> &Descriptor {
        self
    }
}

impl DescriptiveMut for Descriptor {
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        self
    }
}

pub enum DescriptorField {
    Name,
    Keywords,
    Key,
    ShortDescription,
    Description,
    Notes
}

impl DescriptorField {
    pub const CLASSNAME: &'static str = "Descriptor";
    pub const FIELDNAME_NAME: &'static str = "name";
    pub const FIELDNAME_KEYWORDS: &'static str = "keywords";
    pub const FIELDNAME_KEY: &'static str = "key";
    pub const FIELDNAME_SHORT_DESCRIPTION: &'static str = "short_description";
    pub const FIELDNAME_DESCRIPTION: &'static str = "description";
    pub const FIELDNAME_NOTES: &'static str = "notes";

    pub const FIELD_NAME: Field = Field::new(Self::FIELDNAME_NAME, FieldValueType::String);
    pub const FIELD_KEYWORDS: Field = Field::new(Self::FIELDNAME_KEYWORDS, FieldValueType::StringArray);
    pub const FIELD_KEY: Field = Field::new(Self::FIELDNAME_KEY, FieldValueType::String);
    pub const FIELD_SHORT_DESCRIPTION: Field = Field::new(Self::FIELDNAME_SHORT_DESCRIPTION, FieldValueType::String);
    pub const FIELD_DESCRIPTION: Field = Field::new(Self::FIELDNAME_DESCRIPTION, FieldValueType::String);
    pub const FIELD_NOTES: Field = Field::new(Self::FIELDNAME_NOTES, FieldValueType::String);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::Name => &Self::FIELD_NAME,
            Self::Keywords => &Self::FIELD_KEYWORDS,
            Self::Key => &Self::FIELD_KEY,
            Self::ShortDescription => &Self::FIELD_SHORT_DESCRIPTION,
            Self::Description => &Self::FIELD_DESCRIPTION,
            Self::Notes => &Self::FIELD_NOTES
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DescriptorBuilder {
    builder_mode: BuilderMode,
    name: Option<String>,
    keywords: Option<Vec<String>>,
    key: Option<String>,
    short_description: Option<String>,
    description: Option<String>,
    notes: Option<String>
}

impl Builder for DescriptorBuilder {
    type ModelType = Descriptor;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            name: None,
            keywords: None,
            key: None,
            short_description: None,
            description: None,
            notes: None
        }
    }

    fn editor() -> Self {
        Self {
            builder_mode: BuilderMode::Editor,
            ..Self::creator() 
        }
    }

    fn builder_mode(&self) -> BuilderMode {
        self.builder_mode
    }

    fn create(self) -> Result<Creation<Self::BuilderType>> {
        let descriptor = Descriptor {
            name: self.name
                .as_ref()
                .ok_or_else(|| Error::FieldNotSet { class: DescriptorField::CLASSNAME, field: DescriptorField::FIELDNAME_NAME})?
                .clone(),
            keywords: self.keywords
                .as_ref()
                .and_then(|k| Some(k.clone()))
                .unwrap_or_else(|| Vec::new()),
            key: self.key.clone(),
            short_description: self.short_description.clone(),
            description: self.description.clone(),
            notes: self.notes.clone()
        };
        Ok(Creation::new(self, descriptor))
    }

    fn modify(self, original: &mut Descriptor) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Vec::new();

        if let Some(name) = &self.name {
            original.name = name.clone();
            fields_changed.push(DescriptorField::Name.field());
        }
        if self.description.is_some() {
            original.description = self.description.clone();
            fields_changed.push(DescriptorField::Description.field());
        }
        if self.notes.is_some() {
            original.notes = self.notes.clone();
            fields_changed.push(DescriptorField::Notes.field());
        }

        Ok(Modification::new(self, fields_changed))
    }

    fn set(&mut self, raw_field: &str, raw_value: String) -> Result<()> {
        match raw_field {
            DescriptorField::FIELDNAME_NAME => self.name(raw_value),
            DescriptorField::FIELDNAME_DESCRIPTION => self.description(raw_value),
            DescriptorField::FIELDNAME_NOTES => self.notes(raw_value),
            _ => Err(Error::UnknownField{class: DescriptorField::CLASSNAME, field: s!(raw_field)})
        }
    }
}

impl DescriptorBuilder {
    pub fn key(&mut self, key: String) -> Result<()> {
        self.key = Some(key);
        Ok(())
    }

    pub fn name(&mut self, name: String) -> Result<()> {
        self.name = Some(name);
        Ok(())
    }

    pub fn description(&mut self, description: String) -> Result<()> {
        self.description = Some(description);
        Ok(())
    }

    pub fn notes(&mut self, notes: String) -> Result<()> {
        self.notes = Some(notes);
        Ok(())
    }
}

pub trait BuildableDescriptor: Builder {
    fn descriptor(&mut self, descriptor: DescriptorBuilder) -> Result<()>; 
    fn descriptor_builder(&mut self) -> &mut DescriptorBuilder;
}

impl Built for Descriptor {
    type BuilderType = DescriptorBuilder;
}

