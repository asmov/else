use crate::{error::*, builder::*, identity::*, descriptor::*, route::*};
use serde;

/// Connects a single Area to a Route.  
/// All fields are from the point-of-view of the Area, describing the Route that this connects to.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct End {
    /// The identity Area that this end provides an exit/entrance for.
    area_identity: Identity,
    /// The description of the Route that this end connects to, from the point-of-view of the Area.
    descriptor: Descriptor,
    /// The direction that this end is found at, from the point-of-view of the Area.
    direction: Direction
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

impl EndField {
    const CLASSNAME: &'static str = "End";
    const FIELDNAME_AREA_IDENTITY: &'static str = "area_identity";
    const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    const FIELDNAME_DIRECTION: &'static str = "direction";

    const FIELD_AREA_IDENTITY: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_AREA_IDENTITY, FieldValueType::Object);
    const FIELD_DESCRIPTOR: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_DESCRIPTOR, FieldValueType::Object);
    const FIELD_DIRECTION: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_DIRECTION, FieldValueType::Object);
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
        let area_identity = Creation::try_assign(&mut self.area_identity, EndField::AreaIdentity)?;
        let descriptor = Creation::try_assign(&mut self.descriptor, EndField::Descriptor)?;
        let direction = self.direction.as_ref()
            .ok_or_else(|| Error::FieldNotSet { class: EndField::CLASSNAME, field: EndField::FIELDNAME_DIRECTION })?
            .clone();

        let end = End {
            area_identity,
            descriptor,
            direction };

        Ok(Creation::new(self, end))
    }

    fn modify(mut self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Vec::new();

        if self.area_identity.is_some() {
            original.area_identity = Creation::assign(&mut self.area_identity)?;
            fields_changed.push(EndField::AreaIdentity.field())
        }
        if self.descriptor.is_some() {
            original.descriptor = Creation::assign(&mut self.descriptor)?;
            fields_changed.push(EndField::Descriptor.field())
        }
        if let Some(direction) = &self.direction {
            original.direction = direction.clone();
            fields_changed.push(EndField::Direction.field())
        }

        Ok(Modification::new(self, fields_changed))
    }
}

impl Built for End {
    type BuilderType = EndBuilder;
}

impl Descriptive for End {
    /// The description of the Route that this end connects to, from the point-of-view of the Area.
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl DescriptiveMut for End {
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        &mut self.descriptor
    }
}

impl End {
    /// The identity of the Area that this end provides an exit/entrance for.
    pub fn area_identity(&self) -> &Identity {
        &self.area_identity
    } 

    /// The general direction that this end is found within the Area.
    pub fn direction(&self) -> Direction {
        self.direction
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