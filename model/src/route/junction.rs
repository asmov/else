use crate::{error::*, modeling::*, route::*};
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Junction {
    entrances: Vec<End>,
}

impl Built for Junction {
    type BuilderType = JunctionBuilder;
}

impl Junction {
    pub fn entrances(&self) -> &Vec<End> {
        &self.entrances
    }
}

#[derive(Clone, Copy, Debug)]
pub enum JunctionField {
    Entrances,
}

impl Fields for JunctionField {
    fn field(&self) -> &'static Field {
        match self {
            Self::Entrances => &Self::FIELD_ENTRANCES,
        }
    }
}

impl Class for JunctionField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl JunctionField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Junction as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "Junction";
    const FIELDNAME_ENTRANCES: &'static str = "entrances";

    const FIELD_ENTRANCES: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ENTRANCES, FieldValueType::ModelList(EndField::class_ident_const()));
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JunctionBuilder {
    builder_mode: BuilderMode,
    entrances: Vec<ListOp<EndBuilder, UID>>,
}

impl Builder for JunctionBuilder {
    type ModelType = Junction;
    type BuilderType = PointBuilder;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            entrances: Vec::new(),
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
        let mut fields_changed = FieldsChanged::from_builder(&self);

        // at least one entrance is required
        if self.entrances.is_empty() {
            return Err(Error::FieldNotSet { class: JunctionField::CLASSNAME, field: JunctionField::FIELDNAME_ENTRANCES });
        }

        let entrances = Build::create_vec(&mut self.entrances, &mut fields_changed, JunctionField::Entrances)?;

        let junction = Junction {
            entrances,
        };

        Ok(Creation::new(PointBuilder::Junction(self), Point::Junction(junction)))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Build::prepare_modify_composite(&mut self, existing)?;

        if !self.entrances.is_empty() {
            Build::modify_vec(&mut self.entrances, &mut existing.entrances, &mut fields_changed, JunctionField::Entrances)?;
        }

        Ok(Modification::new(PointBuilder::Junction(self), fields_changed))
    }

    fn class_ident(&self) -> &'static ClassIdent {
        JunctionField::class_ident()
    }
}

impl JunctionBuilder {
    pub fn add_entrance(&mut self, end: EndBuilder) -> Result<()> {
        self.entrances.push(ListOp::Add(end));
        Ok(())
    }

    pub fn remove_entrance(&mut self, area_uid: UID) -> Result<()> {
        self.entrances.push(ListOp::Remove(area_uid));
        Ok(())
    }

    pub fn entrances(&self) -> &Vec<ListOp<EndBuilder, UID>> {
        &self.entrances
    }
}

impl point::PointBuilderVariant for JunctionBuilder {
    fn point_builder(self) -> PointBuilder {
        PointBuilder::Junction(self)
    }
}