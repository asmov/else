use crate::{classes::*, builder::*, identity::*, descriptor::*};

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

pub enum ThingViewField {
    Identity,
    Descriptor
}

impl Fields for ThingViewField {
    fn field(&self) -> &'static Field {
        match self {
            Self::Identity => Self::FIELD_IDENTITY,
            Self::Descriptor => Self::FIELD_DESCRIPTOR,
        }
    }
}

impl Class for ThingViewField {
    fn class_id() -> ClassID {
        Self::CLASS_ID
    }

    fn classname() -> &'static str {
        Self::CLASSNAME
    }
}

impl ThingViewField {
    const CLASS_ID: ClassID = ClassIdent::ThingView as ClassID;
    const CLASSNAME: &'static str = "ThingView";
    const FIELD_IDENTITY: &'static Field = &Field::new(Self::CLASS_ID, Self::CLASSNAME, "Identity", FieldValueType::Object);
    const FIELD_DESCRIPTOR: &'static Field = &Field::new(Self::CLASS_ID, Self::CLASSNAME, "Descriptor", FieldValueType::Object);
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

    fn class_id(&self) -> ClassID {
        ThingViewField::class_id()
    }
}