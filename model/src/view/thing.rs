use crate::{error::*, builder::*, codebase::*, descriptor::*, identity::*};

pub struct ThingView {
    uid: UID,
    descriptor: Descriptor
}

impl Keyed for ThingView {
    fn key(&self) -> Option<&str> {
        self.descriptor.key()
    }
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
    UID,
    Descriptor
}

impl Fields for ThingViewField {
    fn field(&self) -> &'static Field {
        match self {
            Self::UID => &Self::FIELD_UID,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
        }
    }
}

impl Class for ThingViewField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl ThingViewField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::ThingView as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "ThingView";
    const FIELDNAME_UID: &'static str = "uid";
    const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    const FIELD_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_UID, FieldValueType::UID);
    const FIELD_DESCRIPTOR: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_DESCRIPTOR, FieldValueType::Model(DescriptorField::class_ident_const()));
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

    fn create(mut self) -> crate::Result<Creation<Self::BuilderType>> {
        let uid = Creation::try_assign(&mut self.identity, ThingViewField::UID)?.to_uid();
        let descriptor = Creation::try_assign(&mut self.descriptor, ThingViewField::Descriptor)?;

        let thing_view = ThingView {
            uid,
            descriptor
        };

        Ok(Creation::new(self, thing_view))
    }

    fn modify(mut self, original: &mut Self::ModelType) -> crate::Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Vec::new();

        if self.descriptor.is_some() {
            Modification::assign(&mut self.descriptor, &mut original.descriptor)?;
            fields_changed.push(ThingViewField::Descriptor.field());
        }

        Ok(Modification::new_old(self, fields_changed))
    }

    fn class_ident(&self) -> &'static ClassIdent {
        ThingViewField::class_ident()
    }
}

impl ThingViewBuilder {
    pub fn identity(&mut self, identity: IdentityBuilder) -> Result<&mut Self> {
        self.identity = Some(identity);
        Ok(self)
    }

    pub fn descriptor(&mut self, descriptor: DescriptorBuilder) -> Result<&mut Self> {
        self.descriptor = Some(descriptor);
        Ok(self)
    }
}