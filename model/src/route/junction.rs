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
    type ModelType = Junction;
    type BuilderType = PointBuilder;

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

    fn create(mut self) -> Result<Creation<Self::BuilderType>> {
        let junction = Junction {
            entrances: Creation::assign_vec(&mut self.entrances)?,
            exit: Creation::try_assign()?,
        };

        Ok(Creation::new(PointBuilder::Junction(self), Point::Junction(junction)))
    }

    fn modify(self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
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

        Ok(Modification::new(PointBuilder::Junction(self), fields_changed))
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