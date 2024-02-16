use crate::{error::*, builder::*, route::*};
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Junction {
    entrances: Vec<End>,
    exit: End,
}

impl Built for Junction {
    type BuilderType = JunctionBuilder;
}

impl Junction {
    pub fn entrances(&self) -> &Vec<End> {
        &self.entrances
    }

    pub fn exit(&self) -> &End {
        &self.exit
    }
}

#[derive(Clone, Copy, Debug)]
pub enum JunctionField {
    Entrances,
    Exit,
}

impl Fields for JunctionField {
    fn field(&self) -> &'static Field {
        match self {
            Self::Entrances => &Self::FIELD_ENTRANCES,
            Self::Exit => &Self::FIELD_EXIT,
        }
    }
}

impl JunctionField {
    const CLASSNAME: &'static str = "Junction";
    const FIELDNAME_ENTRANCES: &'static str = "entrances";
    const FIELDNAME_EXIT: &'static str = "exit";

    const FIELD_ENTRANCES: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_ENTRANCES, FieldValueType::Object);
    const FIELD_EXIT: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_EXIT, FieldValueType::Object);
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JunctionBuilder {
    builder_mode: BuilderMode,
    entrances: Vec<EndBuilder>,
    exit: Option<EndBuilder>
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
        // at least one entrance is required
        if self.entrances.is_empty() {
            return Err(Error::FieldNotSet { class: JunctionField::CLASSNAME, field: JunctionField::FIELDNAME_ENTRANCES });
        }

        let entrances = Creation::assign_vec(&mut self.entrances)?;
        let exit = Creation::try_assign(&mut self.exit, JunctionField::Exit)?;

        let junction = Junction {
            entrances,
            exit
        };

        Ok(Creation::new(PointBuilder::Junction(self), Point::Junction(junction)))
    }

    fn modify(mut self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Vec::new();

        if !self.entrances.is_empty() {
            Creation::modify_vec(&mut self.entrances, &mut original.entrances)?;
            fields_changed.push(JunctionField::Entrances.field())
        }

        if self.exit.is_some() {
            original.exit = Creation::assign(&mut self.exit)?;
            fields_changed.push(JunctionField::Exit.field())
        }

        Ok(Modification::new(PointBuilder::Junction(self), fields_changed))
    }
}

impl JunctionBuilder {
    pub fn add_entrance(&mut self, end: EndBuilder) -> Result<()> {
        self.entrances.push(end);
        Ok(())
    }

    pub fn exit(&mut self, end: EndBuilder) -> Result<()> {
        self.exit = Some(end);
        Ok(())
    }
}

impl point::PointBuilderVariant for JunctionBuilder {
    fn point_builder(self) -> PointBuilder {
        PointBuilder::Junction(self)
    }
}