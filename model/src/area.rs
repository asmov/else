use crate::{codebase::*, error::*, builder::*, identity::*, descriptor::*, thing::*, world::World};
use serde;

/// Represents an area that things are located in, generally. There is no exact position.
/// Each area has a fixed set of `Route` objects that link it to other areas. 
/// There is a dynamic list of `Thing` objects thare are current occupants.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Area {
    uid: UID,
    descriptor: Descriptor,
    route_id_map: Vec<ID>,
    occupant_thing_ids: Vec<UID>
}

impl Keyed for Area {
    fn key(&self) -> Option<&str> {
        self.descriptor.key()
    }
}

impl Identifiable for Area {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl Descriptive for Area {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl Area {
    /// Returns all Thing UIDs currently located here.
    pub fn occupant_ids(&self) -> &Vec<UID> {
        &self.occupant_thing_ids
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

impl Class for AreaField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl AreaField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Area as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "Area";
    const FIELDNAME_IDENTITY: &'static str = "identity";
    const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    const FIELDNAME_ROUTES: &'static str = "routes";
    const FIELDNAME_OCCUPANTS: &'static str = "occupants";

    const FIELD_IDENTITY: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_IDENTITY, FieldValueType::Model);
    const FIELD_DESCRIPTOR: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_DESCRIPTOR, FieldValueType::Model);
    const FIELD_ROUTES: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTES, FieldValueType::VecUID);
    const FIELD_OCCUPANTS: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_OCCUPANTS, FieldValueType::VecUID);
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AreaBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    descriptor: Option<DescriptorBuilder>,
    occupant_thing_ids: Vec<VecOp<UID, UID>>
}

impl Builder for AreaBuilder {
    type ModelType = Area;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            identity: None,
            descriptor: None,
            occupant_thing_ids: Vec::new()
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
        let occupant_thing_ids = self.occupant_thing_ids.iter()
            .map(|op| match op {
                VecOp::Add(uid) => *uid,
                VecOp::Edit(_) => unreachable!("VecOp::Modify not possible in AreaBuilder::create"),
                VecOp::Remove(uid) => unreachable!("VecOp::Remove not possible in AreaBuilder::create") 
            })
            .collect();

        let area = Area {
            uid: identity.into_uid(),
            descriptor,
            route_id_map: Vec::new(), //todo
            occupant_thing_ids
        };

        Ok(Creation::new(self, area))
    }

    fn modify(mut self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Vec::new();

        if self.identity.is_none() {
            self.identity(IdentityBuilder::from_original(&self, original))?;
        }

        if self.descriptor.is_some() {
            let descriptor = self.descriptor.unwrap();
            self.descriptor = Some(descriptor.modify(&mut original.descriptor)?
                .take_builder());
            
            fields_changed.push(AreaField::Descriptor.field());
        }

        if !self.occupant_thing_ids.is_empty() {
            for vecop in &self.occupant_thing_ids {
                match *vecop {
                    VecOp::Add(uid) => original.occupant_thing_ids.push(uid),
                    VecOp::Edit(_) => unreachable!("VecOp::Modify not possible in AreaBuilder::modify"),
                    VecOp::Remove(uid) => {
                        let index = original.occupant_thing_ids.iter().position(|&x| x == uid)
                            .ok_or_else(|| Error::ModelNotFoundFor{model: "Thing", uid, op: "AreaBuilder::remove_occupant()"})?;
                        original.occupant_thing_ids.remove(index);
                    }
                }
            }
        }

        Ok(Modification::new(self, fields_changed))
    }

    fn sync_modify(self, world: &mut World) -> Result<Modification<Self::BuilderType>> {
        let area_uid = self.get_identity().unwrap().get_uid()?;
        let area_dog_house_mut = world.area_mut(area_uid).unwrap(); //todo: don't unwrap
        self.modify(area_dog_house_mut)
    }

    fn class_id(&self) -> ClassID {
        AreaField::class_id()
    }
}

impl MaybeIdentifiable for AreaBuilder {
    fn try_uid(&self) -> Result<UID> {
        Self::_try_uid(&self)
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
    fn edit_area(&mut self, area: AreaBuilder) -> Result<()>; 
    fn remove_area(&mut self, area_uid: UID) -> Result<()>; 
}

impl AreaBuilder {
    pub fn add_occupant(&mut self, thing_uid: UID) -> Result<&mut Self> {
        self.occupant_thing_ids.push(VecOp::Add(thing_uid));
        Ok(self)
    }

    pub fn remove_occupant(&mut self, thing_uid: UID) -> Result<&mut Self> {
        assert!(self.builder_mode() == BuilderMode::Editor, "AreaBuilder::remove_occupant only allowed in Editor mode");
        self.occupant_thing_ids.push(VecOp::Remove(thing_uid));
        Ok(self)
    }
 }

