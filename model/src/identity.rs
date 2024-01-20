use crate::{error::*, builder::*};
use serde;

pub type ID = u64;
pub type RegionID = u16;
pub type ZoneID = u16;
pub type UniverseID = u32;

#[derive(PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
pub struct Identity {
    id: ID,
    region_id: RegionID,
    zone_id: ZoneID,
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

    fn zone_id(&self) -> ZoneID {
        self.identity().zone_id
    }

    fn universe_id(&self) -> UniverseID {
        self.identity().universe_id
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
    ZoneID,
    UniverseID,
}

impl IdentityField {
    pub const CLASSNAME: &'static str = "Identity";
    pub const FIELDNAME_ID: &'static str = "id";
    pub const FIELDNAME_REGION_ID: &'static str = "region_id";
    pub const FIELDNAME_ZONE_ID: &'static str = "zone_id";
    pub const FIELDNAME_UNIVERSE_ID: &'static str = "universe_id";

    pub const FIELD_ID: Field = Field::new(Self::FIELDNAME_ID, FieldValueType::String);
    pub const FIELD_REGION_ID: Field = Field::new(Self::FIELDNAME_REGION_ID, FieldValueType::StringArray);
    pub const FIELD_ZONE_ID: Field = Field::new(Self::FIELDNAME_ZONE_ID, FieldValueType::String);
    pub const FIELD_UNIVERSE_ID: Field = Field::new(Self::FIELDNAME_UNIVERSE_ID, FieldValueType::String);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::ID => &Self::FIELD_ID,
            Self::RegionID => &Self::FIELD_REGION_ID,
            Self::ZoneID => &Self::FIELD_ZONE_ID,
            Self::UniverseID => &Self::FIELD_UNIVERSE_ID
        }
    }
}

#[derive(Clone, Debug)]
pub struct IdentityBuilder {
    builder_mode: BuilderMode,
    id: Option<ID>,
    region_id: Option<RegionID>,
    zone_id: Option<ZoneID>,
    universe_id: Option<UniverseID>
}

impl Builder for IdentityBuilder {
    type Type = Identity;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            id: None,
            region_id: None,
            zone_id: None,
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

    fn create(self) -> Result<Self::Type> {
        Ok(Identity {
            id: self.id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::FIELDNAME_ID})?,
            region_id: self.region_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::FIELDNAME_REGION_ID})?,
            zone_id: self.zone_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::FIELDNAME_ZONE_ID})?,
            universe_id: self.universe_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::FIELDNAME_UNIVERSE_ID})?
        })
    }

    fn modify(self, original: &mut Self::Type) -> Result<ModifyResult> {
        let mut fields_changed = Vec::new();

        if let Some(id) = self.id {
            original.id = id;
            fields_changed.push(IdentityField::ID.field());
        }
        if let Some(region_id) = self.region_id {
            original.region_id = region_id;
            fields_changed.push(IdentityField::RegionID.field());
        }
        if let Some(zone_id) = self.zone_id {
            original.zone_id = zone_id;
            fields_changed.push(IdentityField::ZoneID.field());
        }
        if let Some(universe_id) = self.universe_id {
            original.universe_id = universe_id;
            fields_changed.push(IdentityField::UniverseID.field());
        }

        Ok(ModifyResult::new(fields_changed))
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

    pub fn zone_id(&mut self, zone_id: ZoneID) -> Result<()> {
        self.zone_id = Some(zone_id);
        Ok(())
    }

    pub fn universe_id(&mut self, universe_id: UniverseID) -> Result<()> {
        self.universe_id = Some(universe_id);
        Ok(())
    }

    pub fn guid(&mut self, id: ID, region_id: RegionID, zone_id: ZoneID, universe_id: UniverseID) -> Result<()> {
        self.id(id)?;
        self.region_id(region_id)?;
        self.zone_id(zone_id)?;
        self.universe_id(universe_id)?;
        Ok(())
    }

    pub fn get_id(&self) -> Option<ID> {
        self.id
    }

    pub fn get_region_id(&self) -> Option<RegionID> {
        self.region_id
    }

    pub fn get_zone_id(&self) -> Option<ZoneID> {
        self.zone_id
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
    pub fn new(id: ID, region_id: RegionID, zone_id: ZoneID, universe_id: UniverseID) -> Self {
        Self {
            id: id,
            region_id: region_id,
            zone_id: zone_id,
            universe_id: universe_id,
        }
    }

    pub fn to_creator(&self) -> IdentityBuilder {
        let mut creator = Identity::creator();
        creator.guid(self.id, self.region_id, self.zone_id, self.universe_id);
        creator
    }
}

