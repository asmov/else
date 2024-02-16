use crate::{builder::*, descriptor::{self, *}, entity::*, error::*, identity::*, world::World};
use serde;

/// Represents an area that things are located in, generally. There is no exact position.
/// Each area has a fixed set of `Route` objects that link it to other areas. 
/// There is a dynamic list of `Thing` objects thare are current occupants.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Area {
    identity: Identity,
    descriptor: Descriptor,
    route_id_map: Vec<ID>,
    occupant_thing_ids: Vec<ID>
}

impl Identifiable for Area {
    fn identity(&self) -> &Identity {
        &self.identity
    }
}

impl IdentifiableMut for Area {
    fn identity_mut(&mut self) -> &mut Identity {
        &mut self.identity
    }
}

impl Descriptive for Area {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl DescriptiveMut for Area {
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        &mut self.descriptor
    }
}

#[derive(Debug)]
pub enum AreaField {
    Identity,
    Descriptor,
    Routes,
    Occupants,
}

impl Fields for AreaField {
    fn field(&self) -> &'static Field {
        match self {
            Self::Identity => &Self::FIELD_IDENTITY,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::Routes => &Self::FIELD_ROUTES,
            Self::Occupants => &Self::FIELD_OCCUPANTS,
        }
    }
}

impl AreaField {
    const CLASSNAME: &'static str = "Area";
    const FIELDNAME_IDENTITY: &'static str = "identity";
    const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    const FIELDNAME_ROUTES: &'static str = "routes";
    const FIELDNAME_OCCUPANTS: &'static str = "occupants";

    const FIELD_IDENTITY: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_IDENTITY, FieldValueType::Object);
    const FIELD_DESCRIPTOR: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_DESCRIPTOR, FieldValueType::Object);
    const FIELD_ROUTES: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_ROUTES, FieldValueType::ObjectIDArray);
    const FIELD_OCCUPANTS: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_OCCUPANTS, FieldValueType::ObjectIDArray);
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AreaBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    descriptor: Option<DescriptorBuilder>,
}

impl Builder for AreaBuilder {
    type ModelType = Area;
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

    fn create(mut self) -> Result<Creation<Self::BuilderType>> {
        let identity = Creation::try_assign(&mut self.identity, AreaField::Identity)?;
        let descriptor = Creation::try_assign(&mut self.descriptor, AreaField::Descriptor)?;

        let area = Area {
            identity,
            descriptor,
            route_id_map: Vec::new(), //todo
            occupant_thing_ids: Vec::new(), //todo
        };

        Ok(Creation::new(self, area))
    }

    fn modify(mut self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Vec::new();

        if self.identity.is_none() {
            self.identity(original.identity.editor_clone())?;
        }

        if self.descriptor.is_some() {
            let descriptor = self.descriptor.unwrap();
            self.descriptor = Some(descriptor.modify(&mut original.descriptor)?
                .take_builder());
            
            fields_changed.push(AreaField::Descriptor.field());
        }

        Ok(Modification::new(self, fields_changed))
    }

    fn sync_modify(self, world: &mut World) -> Result<Modification<Self::BuilderType>> {
        let area_id = self.get_identity().unwrap().get_id().unwrap();
        let area_dog_house_mut = world.area_mut(area_id).unwrap(); //todo: don't unwrap
        self.modify(area_dog_house_mut)
    }
}

impl BuildableIdentity for AreaBuilder {
    fn identity(&mut self, identity: IdentityBuilder) -> Result<()> {
        self.identity = Some(identity);
        Ok(())
    }

    fn identity_builder(&mut self) -> &mut IdentityBuilder {
        if self.identity.is_none() {
            self.identity = Some(Identity::builder(self.builder_mode()))
        }

        self.identity.as_mut().unwrap()
    }

    fn get_identity(&self) -> Option<&IdentityBuilder> {
        self.identity.as_ref()
    }
}

impl BuildableDescriptor for AreaBuilder {
    fn descriptor(&mut self, descriptor: DescriptorBuilder) -> Result<()> {
        self.descriptor = Some(descriptor);
        Ok(())
    }

    fn descriptor_builder(&mut self) -> &mut DescriptorBuilder {
        if self.descriptor.is_none() {
            self.descriptor = Some(Descriptor::builder(self.builder_mode()))
        }

        self.descriptor.as_mut().unwrap()
    }
}

impl Built for Area {
    type BuilderType = AreaBuilder;
}

pub trait BuildableAreaVector {
    fn add_area(&mut self, area: AreaBuilder) -> Result<()>; 
}

