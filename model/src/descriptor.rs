use serde;
use crate::{modeling::*, codebase::*, error::*, identity::*, world::*};

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

impl Keyed for Descriptor {
    fn key(&self) -> Option<&str> {
        self.key.as_ref().map(|s| s.as_str())
    }
}

/// The trait that provides standard immutable access to a `Descriptor` struct
pub trait Descriptive: Keyed {
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

    /*fn key(&self) -> Option<&String> {
        self.descriptor().key.as_ref()
    }*/

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

impl Descriptive for Descriptor {
    fn descriptor(&self) -> &Descriptor {
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

impl Class for DescriptorField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl Fields for DescriptorField {
    fn field(&self) -> &'static Field {
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

impl TryFrom<&str> for DescriptorField {
    type Error = Error;
    
    fn try_from(name: &str) -> Result<Self> {
        match name {
            "name" => Ok(Self::Name),
            "keywords" => Ok(Self::Keywords),
            "key" => Ok(Self::Key),
            "short_description" => Ok(Self::ShortDescription),
            "description" => Ok(Self::Description),
            "notes" => Ok(Self::Notes),
            _ => Err(Error::UnknownField {class: Self::CLASSNAME, field: name.to_string()})
        }
    }
}

impl DescriptorField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Descriptor as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "Descriptor";
    const FIELD_NAME: Field = Field::new(&Self::CLASS_IDENT, "name", FieldValueType::String);
    const FIELD_KEYWORDS: Field = Field::new(&Self::CLASS_IDENT, "keywords", FieldValueType::StringList);
    const FIELD_KEY: Field = Field::new(&Self::CLASS_IDENT, "key", FieldValueType::String);
    const FIELD_SHORT_DESCRIPTION: Field = Field::new(&Self::CLASS_IDENT, "short_description", FieldValueType::String);
    const FIELD_DESCRIPTION: Field = Field::new(&Self::CLASS_IDENT, "description", FieldValueType::String);
    const FIELD_NOTES: Field = Field::new(&Self::CLASS_IDENT, "notes", FieldValueType::String);

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
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
    type DomainType = World;
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
                .ok_or_else(|| Error::FieldNotSet { class: DescriptorField::CLASSNAME, field: "name"})?
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

    fn modify(self, existing: &mut Descriptor) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Vec::new();

        if let Some(name) = &self.name {
            existing.name = name.clone();
            fields_changed.push(DescriptorField::Name.field());
        }
        if self.description.is_some() {
            existing.description = self.description.clone();
            fields_changed.push(DescriptorField::Description.field());
        }
        if self.notes.is_some() {
            existing.notes = self.notes.clone();
            fields_changed.push(DescriptorField::Notes.field());
        }

        Ok(Modification::new_old(self, fields_changed))
    }

    fn set(&mut self, field_name: &str, raw_value: String) -> Result<()> {
        match DescriptorField::try_from(field_name)? {
            DescriptorField::Name => self.name(raw_value)?,
            DescriptorField::Keywords => {
                self.keywords(
                    raw_value.split_whitespace()
                        .map(|s| s.to_owned())
                        .collect())?
            },
            DescriptorField::Key => self.key(raw_value)?,
            DescriptorField::ShortDescription => self.short_description(raw_value)?,
            DescriptorField::Description => self.description(raw_value)?,
            DescriptorField::Notes => self.notes(raw_value)?,
        };

        Ok(())
    }

    fn class_ident(&self) -> &'static ClassIdent {
        DescriptorField::class_ident()
    }
}

impl DescriptorBuilder {
    pub fn key(&mut self, key: String) -> Result<&mut Self> {
        self.key = Some(key);
        Ok(self)
    }

    pub fn keywords(&mut self, keywords: Vec<String>) -> Result<&mut Self> {
        self.keywords = Some(keywords);
        Ok(self)
    }


    pub fn name(&mut self, name: String) -> Result<&mut Self> {
        self.name = Some(name);
        Ok(self)
    }

    pub fn description(&mut self, description: String) -> Result<&mut Self> {
        self.description = Some(description);
        Ok(self)
    }

    pub fn short_description(&mut self, description: String) -> Result<&mut Self> {
        self.short_description = Some(description);
        Ok(self)
    }

    pub fn notes(&mut self, notes: String) -> Result<&mut Self> {
        self.notes = Some(notes);
        Ok(self)
    }
}

pub trait BuildableDescriptor: Builder {
    fn descriptor(&mut self, descriptor: DescriptorBuilder) -> Result<()>; 
    fn descriptor_builder(&mut self) -> &mut DescriptorBuilder;
}

impl Built for Descriptor {
    type BuilderType = DescriptorBuilder;
}

