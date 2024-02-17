use crate::{builder::*, identity::*, descriptor::*};

pub struct ThingView {
    uid: UID,
    descriptor: Descriptor
}

impl Identifiable for ThingView {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl Descriptive for ThingView {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl Built for ThingView {
    type BuilderType = ThingViewBuilder;
}

pub enum ThingViewFields {
    Identity,
    Descriptor
}

impl Fields for ThingViewFields {
    fn field(&self) -> &'static Field {
        match self {
            Self::Identity => Self::FIELD_IDENTITY,
            Self::Descriptor => Self::FIELD_DESCRIPTOR,
        }
    }
}

impl ThingViewFields {
    const CLASSNAME: &'static str = "ThingView";
    const FIELD_IDENTITY: &'static Field = &Field::new(Self::CLASSNAME, "Identity", FieldValueType::Object);
    const FIELD_DESCRIPTOR: &'static Field = &Field::new(Self::CLASSNAME, "Descriptor", FieldValueType::Object);
}

pub struct ThingViewBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    descriptor: Option<DescriptorBuilder>
}

impl Builder for ThingViewBuilder {
    type ModelType = ThingView;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            identity: None,
            descriptor: None
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

    fn create(self) -> crate::Result<Creation<Self::BuilderType>> {
       todo!() 
    }

    fn modify(self, original: &mut Self::ModelType) -> crate::Result<Modification<Self::BuilderType>> {
        todo!()
    }
}