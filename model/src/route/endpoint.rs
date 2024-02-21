use crate::{error::*, builder::*, route::*};
use serde;

/// Connects a single Area to a Route.  
/// All fields are from the point-of-view of the Area, describing the Route that this connects to.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Endpoint {
    end: End
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

    const FIELD_END: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_END, FieldValueType::Model);
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
        let end = Creation::try_assign(&mut self.end, EndpointField::End)?;

        let endpoint = Endpoint { end };

        Ok(Creation::new(PointBuilder::Endpoint(self), Point::Endpoint(endpoint)))
    }

    fn modify(mut self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Vec::new();

        if self.end.is_some() {
            original.end = Creation::assign(&mut self.end)?;
            fields_changed.push(EndpointField::End.field())
        }

        Ok(Modification::new(PointBuilder::Endpoint(self), fields_changed))
    }

    fn class_id(&self) -> ClassID {
        EndpointField::class_id()
    }
}

impl Built for Endpoint {
    type BuilderType = EndpointBuilder;
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
}

impl point::PointBuilderVariant for EndpointBuilder {
    fn point_builder(self) -> PointBuilder {
        PointBuilder::Endpoint(self)
    }
}