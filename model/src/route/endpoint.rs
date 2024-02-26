use crate::{error::*, modeling::*, route::*};
use serde;

/// Connects a single Area to a Route.  
/// All fields are from the point-of-view of the Area, describing the Route that this connects to.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Endpoint {
    end: End
}

impl Endpoint {
    pub fn end(&self) -> &End {
        &self.end
    }
}

#[derive(Clone, Copy, Debug)]
pub enum EndpointField {
    End
}

impl Fields for EndpointField {
    fn field(&self) -> &'static Field {
        match self {
            Self::End=> &Self::FIELD_END,
        }
    }
}

impl Class for EndpointField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl EndpointField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Endpoint as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "Endpoint";
    const FIELDNAME_END: &'static str = "end";

    const FIELD_END: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_END, FieldValueType::Model(EndField::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EndpointBuilder {
    builder_mode: BuilderMode,
    end: Option<EndBuilder>
}

impl Builder for EndpointBuilder {
    type ModelType = Endpoint;
    type BuilderType = PointBuilder;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            end: None
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

        let end = Build::create(&mut self.end, &mut fields_changed, EndpointField::End)?;

        let endpoint = Endpoint {
            end
        };

        Ok(Creation::new(PointBuilder::Endpoint(self), Point::Endpoint(endpoint)))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Build::prepare_modify_composite(&mut self, existing)?;

        if self.end.is_some() {
            Build::modify(&mut self.end, &mut existing.end, &mut fields_changed, EndpointField::End)?;
        }

        Ok(Modification::new(PointBuilder::Endpoint(self), fields_changed))
    }

    fn class_ident(&self) -> &'static ClassIdent {
        EndpointField::class_ident()
    }
}

impl Built for Endpoint {
    type BuilderType = EndpointBuilder;
}

impl CloneBuilding for EndpointBuilder {
    fn clone_model(builder_mode: BuilderMode, existing: &Self::ModelType) -> Self {
        Self {
            builder_mode,
            end: Some(EndBuilder::clone_model(builder_mode, &existing.end))
        }
    }
}

impl EndpointBuilder {
    pub fn end(&mut self, end: EndBuilder) -> Result<()> {
        self.end = Some(end);
        Ok(())
    }

    pub fn end_builder(&mut self) -> &mut EndBuilder {
        if self.end.is_none() {
            self.end = Some(End::builder(self.builder_mode()));
        }

        self.end.as_mut().unwrap()
    }

    pub fn get_end_builder(&self) -> Option<&EndBuilder> {
        self.end.as_ref()
    }
}

impl point::PointBuilderVariant for EndpointBuilder {
    fn point_builder(self) -> PointBuilder {
        PointBuilder::Endpoint(self)
    }
}