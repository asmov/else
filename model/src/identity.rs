use crate::{error::*, builder::*};
use serde;

pub type ID = u64;
pub type RegionID = u32;
pub type WorldID = u16;
pub type UniverseID = u16;

#[derive(PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
pub struct Identity {
    id: ID,
    region_id: RegionID,
    world_id: WorldID,
    universe_id: UniverseID,
}

pub trait Identifiable {
    fn identity(&self) -> &Identity;

    fn id(&self) -> ID {
        self.identity().id
    }

    fn region_id(&self) -> RegionID {
        self.identity().region_id
    }

    fn world_id(&self) -> WorldID {
        self.identity().world_id
    }

    fn universe_id(&self) -> UniverseID {
        self.identity().universe_id
    }

    fn editor_clone(&self) -> IdentityBuilder {
        let mut editor = IdentityBuilder::editor();
        editor.id(self.id());
        editor.region_id(self.region_id());
        editor.world_id(self.world_id());
        editor.universe_id(self.universe_id());
        editor
    }
}

pub trait IdentifiableMut: Identifiable {
    fn identity_mut(&mut self) -> &mut Identity;
}

impl Identifiable for Identity {
    fn identity(&self) -> &Identity {
        self
    }
}

impl IdentifiableMut for Identity {
    fn identity_mut(&mut self) -> &mut Identity {
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub enum IdentityField {
    ID,
    RegionID,
    WorldID,
    UniverseID,
}

impl IdentityField {
    pub const CLASSNAME: &'static str = "Identity";
    pub const FIELDNAME_ID: &'static str = "id";
    pub const FIELDNAME_REGION_ID: &'static str = "region_id";
    pub const FIELDNAME_WORLD_ID: &'static str = "world_id";
    pub const FIELDNAME_UNIVERSE_ID: &'static str = "universe_id";

    pub const FIELD_ID: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_ID, FieldValueType::String);
    pub const FIELD_REGION_ID: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_REGION_ID, FieldValueType::StringArray);
    pub const FIELD_WORLD_ID: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_WORLD_ID, FieldValueType::String);
    pub const FIELD_UNIVERSE_ID: Field = Field::new(Self::CLASSNAME, Self::FIELDNAME_UNIVERSE_ID, FieldValueType::String);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::ID => &Self::FIELD_ID,
            Self::RegionID => &Self::FIELD_REGION_ID,
            Self::WorldID => &Self::FIELD_WORLD_ID,
            Self::UniverseID => &Self::FIELD_UNIVERSE_ID
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IdentityBuilder {
    builder_mode: BuilderMode,
    id: Option<ID>,
    region_id: Option<RegionID>,
    world_id: Option<WorldID>,
    universe_id: Option<UniverseID>
}

impl Builder for IdentityBuilder {
    type ModelType = Identity;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            id: None,
            region_id: None,
            world_id: None,
            universe_id: None
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

    fn create(self) -> Result<Creation<Self::BuilderType>> {
        let identity = Identity {
            id: self.id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::FIELDNAME_ID})?,
            region_id: self.region_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::FIELDNAME_REGION_ID})?,
            world_id: self.world_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::FIELDNAME_WORLD_ID})?,
            universe_id: self.universe_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::FIELDNAME_UNIVERSE_ID})?
        };
        Ok(Creation::new(self, identity))
    }

    fn modify(self, original: &mut Self::ModelType) -> Result<Modification<Self>> {
        let mut fields_changed = Vec::new();

        if let Some(id) = self.id {
            original.id = id;
            fields_changed.push(IdentityField::ID.field());
        }
        if let Some(region_id) = self.region_id {
            original.region_id = region_id;
            fields_changed.push(IdentityField::RegionID.field());
        }
        if let Some(world_id) = self.world_id {
            original.world_id = world_id;
            fields_changed.push(IdentityField::WorldID.field());
        }
        if let Some(universe_id) = self.universe_id {
            original.universe_id = universe_id;
            fields_changed.push(IdentityField::UniverseID.field());
        }

        Ok(Modification::new(self, fields_changed))
    }
}

impl IdentityBuilder {
    pub fn id(&mut self, id: ID) -> Result<()> {
        self.id = Some(id);
        Ok(())
    }
    
    pub fn region_id(&mut self, region_id: RegionID) -> Result<()> {
        self.region_id = Some(region_id);
        Ok(())
    }

    pub fn world_id(&mut self, world_id: WorldID) -> Result<()> {
        self.world_id = Some(world_id);
        Ok(())
    }

    pub fn universe_id(&mut self, universe_id: UniverseID) -> Result<()> {
        self.universe_id = Some(universe_id);
        Ok(())
    }

    pub fn guid(&mut self, id: ID, region_id: RegionID, world_id: WorldID, universe_id: UniverseID) -> Result<()> {
        self.id(id)?;
        self.region_id(region_id)?;
        self.world_id(world_id)?;
        self.universe_id(universe_id)?;
        Ok(())
    }

    pub fn get_id(&self) -> Option<ID> {
        self.id
    }

    pub fn get_region_id(&self) -> Option<RegionID> {
        self.region_id
    }

    pub fn get_world_id(&self) -> Option<WorldID> {
        self.world_id
    }

    pub fn get_universe_id(&self) -> Option<UniverseID> {
        self.universe_id
    }
}

pub trait BuildableIdentity: Builder {
    fn identity(&mut self, identity: IdentityBuilder) -> Result<()>; 
    fn identity_builder(&mut self) -> &mut IdentityBuilder;
    fn get_identity(&self) -> Option<&IdentityBuilder>;

    fn has_identity(&self) -> bool {
        self.get_identity().is_some()
    }
}

impl Built for Identity {
    type BuilderType = IdentityBuilder;
}

impl Identity {
    pub fn new(id: ID, region_id: RegionID, world_id: WorldID, universe_id: UniverseID) -> Self {
        Self {
            id,
            region_id,
            world_id,
            universe_id,
        }
    }

    pub fn to_creator(&self) -> IdentityBuilder {
        let mut creator = Identity::creator();
        creator.guid(self.id, self.region_id, self.world_id, self.universe_id).unwrap();
        creator
    }
}

