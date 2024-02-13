use crate::{error::*, builder::*, route::*};
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Junction {
    entrances: Vec<Endpoint>,
    exit: Endpoint,
}

impl Built for Junction {
    type BuilderType = JunctionBuilder;
}

impl Junction {
    pub fn entrances(&self) -> &Vec<Endpoint> {
        &self.entrances
    }

    pub fn exit(&self) -> &Endpoint {
        &self.exit
    }
}

#[derive(Clone, Copy, Debug)]
pub enum JunctionField {
    Entrances,
    Exit,
}

impl JunctionField {
    pub const CLASSNAME: &'static str = "Junction";
    pub const FIELDNAME_ENTRANCES: &'static str = "entrances";
    pub const FIELDNAME_EXIT: &'static str = "exit";

    pub const FIELD_ENTRANCES: Field = Field::new(Self::FIELDNAME_ENTRANCES, FieldValueType::Object);
    pub const FIELD_EXIT: Field = Field::new(Self::FIELDNAME_EXIT, FieldValueType::Object);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::Entrances => &Self::FIELD_ENTRANCES,
            Self::Exit => &Self::FIELD_EXIT,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JunctionBuilder {
    builder_mode: BuilderMode,
    entrances: Vec<EndpointBuilder>,
    exit: Option<EndpointBuilder>
}

impl Builder for JunctionBuilder {
    type Type = Junction;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            entrances: Vec::new(),
            exit: None
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

    fn create(self) -> Result<Self::Type> {
        Ok(Junction {
            entrances: self.entrances.into_iter()
                .map(|entrance| entrance.create())
                .collect::<Result<Vec<_>>>()?,
            exit: self.exit
                .ok_or_else(||
                    Error::FieldNotSet {class: JunctionField::CLASSNAME, field: JunctionField::FIELDNAME_EXIT})?
                .create()?,

        })
    }

    fn modify(self, original: &mut Self::Type) -> Result<Modification<Self>> {
        let mut fields_changed = Vec::new();

        if !self.entrances.is_empty() {
            fields_changed.push(JunctionField::Entrances.field())
        }

        for entrance in self.entrances {
            original.entrances.push(entrance.create()?);
        }

        if let Some(exit) = self.exit {
            original.exit = exit.create()?;
            fields_changed.push(JunctionField::Exit.field())
        }

        Ok(Modification::new(self, fields_changed))
    }
}

impl JunctionBuilder {
    pub fn add_entrance(&mut self, endpoint: EndpointBuilder) -> Result<()> {
        self.entrances.push(endpoint);
        Ok(())
    }
}

impl point::BuildablePoint for JunctionBuilder {
    fn point_builder(self) -> PointBuilder {
        PointBuilder::Junction(self)
    }
}