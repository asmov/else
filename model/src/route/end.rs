use crate::{error::*, modeling::*, identity::*, descriptor::*, route::*};
use serde;

/// Connects a single Area to a Route.  
/// All fields are from the point-of-view of the Area, describing the Route that this connects to.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct End {
    /// The Area that this end provides an exit/entrance for.
    area_uid: UID,
    /// The description of the Route that this end connects to, from the point-of-view of the Area.
    descriptor: Descriptor,
    /// The direction that this end is found at, from the point-of-view of the Area.
    direction: Direction
}

impl Keyed for End {
    fn key(&self) -> Option<&str> {
        self.descriptor.key()
    }
}

impl Identifiable for End {
    fn uid(&self) -> UID {
        self.area_uid.uid()
    }
}

impl Descriptive for End {
    /// The description of the Route that this end connects to, from the point-of-view of the Area.
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl Built for End {
    type BuilderType = EndBuilder;
}

impl End {
    pub fn area_uid(&self) -> UID {
        self.area_uid
    } 

    pub fn direction(&self) -> Direction {
        self.direction
    }
}

#[derive(Clone, Copy, Debug)]
pub enum EndField {
    AreaIdentity,
    Descriptor,
    Direction
}

impl Fields for EndField {
    fn field(&self) -> &'static Field {
        match self {
            Self::AreaIdentity => &Self::FIELD_AREA_IDENTITY,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::Direction => &Self::FIELD_DIRECTION
        }
    }
}

impl Class for EndField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl EndField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::End as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "End";
    const FIELDNAME_AREA_IDENTITY: &'static str = "area_identity";
    const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    const FIELDNAME_DIRECTION: &'static str = "direction";

    const FIELD_AREA_IDENTITY: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_AREA_IDENTITY, FieldValueType::Model(IdentityField::class_ident_const()));
    const FIELD_DESCRIPTOR: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_DESCRIPTOR, FieldValueType::Model(DescriptorField::class_ident_const()));
    const FIELD_DIRECTION: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_DIRECTION, FieldValueType::NonPrimitive(Direction::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EndBuilder {
    builder_mode: BuilderMode,
    area_identity: Option<IdentityBuilder>,
    descriptor: Option<DescriptorBuilder>,
    direction: Option<Direction>
}

impl Builder for EndBuilder {
    type ModelType = End;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            area_identity: None,
            descriptor: None,
            direction: None
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

        let area_uid = Build::create(&mut self.area_identity, &mut fields_changed, EndField::AreaIdentity)?.uid();
        let descriptor = Build::create(&mut self.descriptor, &mut fields_changed, EndField::Descriptor)?;
        let direction = Build::create_value(&mut self.direction, &mut fields_changed, EndField::Direction)?;

        let end = End {
            area_uid,
            descriptor,
            direction
        };

        Ok(Creation::new(self, end))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Build::prepare_modify_composite(&mut self, existing)?;

        if self.descriptor.is_some() {
            Build::modify(&mut self.descriptor, &mut existing.descriptor, &mut fields_changed, EndField::Descriptor)?;
        }
        if self.direction.is_some() { 
            existing.direction = Build::modify_value(&self.direction, &mut fields_changed, EndField::Descriptor)?;
        }

        Ok(Modification::new(self, fields_changed))
    }

    fn class_ident(&self) -> &'static ClassIdent {
        EndField::class_ident()
    }
}

impl BuildableDescriptor for EndBuilder {
    fn descriptor(&mut self, descriptor: DescriptorBuilder) -> Result<()> {
        self.descriptor = Some(descriptor);
        Ok(())
    }

    fn descriptor_builder(&mut self) -> &mut DescriptorBuilder {
        if self.descriptor.is_none() {
            self.descriptor = Some(Descriptor::builder(self.builder_mode()));
        }

        self.descriptor.as_mut().unwrap()
    }
}

impl MaybeIdentifiable for EndBuilder {
    fn try_uid(&self) -> Result<UID> {
        self.area_identity.as_ref()
            .ok_or_else(|| Error::BuildableUID{})
            .and_then(|uid| uid.try_uid())
    }
}

impl EndBuilder {
    pub fn area_identity(&mut self, id: IdentityBuilder) -> Result<()> {
        self.area_identity = Some(id);
        Ok(())
    }

    pub fn area_identity_builder(&mut self) -> &mut IdentityBuilder {
        if self.area_identity.is_none() {
            self.area_identity = Some(Identity::builder(self.builder_mode()));
        }

        self.area_identity.as_mut().unwrap()
    }

    pub fn get_area_identity(&self) -> Option<&IdentityBuilder> {
        self.area_identity.as_ref()
    }

    pub fn direction(&mut self, direction: Direction) -> Result<()> {
        self.direction = Some(direction);
        Ok(())
    }
}