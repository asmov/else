use serde;
use crate::{error::*, builder::*};

pub type UID        = u128;
pub type UniverseID = u16;
pub type WorldID    = u16;
pub type RegionID   = u32;
pub type ID         = u64;

const UID_BITS:         usize = std::mem::size_of::<UID>()         * 8;
const UNIVERSE_ID_BITS: usize = std::mem::size_of::<UniverseID>()  * 8;
const WORLD_ID_BITS:    usize = std::mem::size_of::<WorldID>()     * 8;
const REGION_ID_BITS:   usize = std::mem::size_of::<RegionID>()    * 8;
const ID_BITS:          usize = std::mem::size_of::<ID>()          * 8;

const UNIVERSE_ID_SHIFT:    usize = UID_BITS          - UNIVERSE_ID_BITS;
const WORLD_ID_SHIFT:       usize = UNIVERSE_ID_SHIFT - WORLD_ID_BITS;
const REGION_ID_SHIFT:      usize = WORLD_ID_SHIFT    - REGION_ID_BITS;
const ID_SHIFT:             usize = REGION_ID_SHIFT   - ID_BITS;

#[derive(PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
pub struct Identity {
    universe_id: UniverseID,
    world_id: WorldID,
    region_id: RegionID,
    id: ID
}

pub trait Identifiable {
    fn identity(&self) -> &Identity;

    fn uid(&self) -> UID {
        self.identity().to_uid()
    }

    fn universe_id(&self) -> UniverseID {
        self.identity().universe_id
    }

    fn world_id(&self) -> WorldID {
        self.identity().world_id
    }

    fn region_id(&self) -> RegionID {
        self.identity().region_id
    }

    fn id(&self) -> ID {
        self.identity().id
    }

    //todo: remove
    fn editor_clone(&self) -> IdentityBuilder {
        let mut editor = IdentityBuilder::editor();
        editor.id(self.id());
        editor.region_id(self.region_id());
        editor.world_id(self.world_id());
        editor.universe_id(self.universe_id());
        editor
    }
}

impl Identifiable for Identity {
    fn identity(&self) -> &Identity {
        self
    }
}

impl Built for Identity {
    type BuilderType = IdentityBuilder;
}

impl Identity {
    pub fn new(universe_id: UniverseID, world_id: WorldID, region_id: RegionID, id: ID) -> Self {
        Self {
            universe_id,
            world_id,
            region_id,
            id
        }
    }

    pub fn to_creator(&self) -> IdentityBuilder {
        let mut creator = Identity::creator();
        creator.all(self.universe_id, self.world_id, self.region_id, self.id).unwrap();
        creator
    }

    pub const fn from_uid(value: UID) -> Self {
        Self {
            universe_id:    (value >> UNIVERSE_ID_SHIFT)    as UniverseID,
            world_id:       (value >> WORLD_ID_SHIFT)       as WorldID,
            region_id:      (value >> REGION_ID_SHIFT)      as RegionID,
            id:             (value >> ID_SHIFT)             as ID 
        }
    }

    pub const fn to_uid(&self) -> UID {
        0
        | ((self.universe_id as UID) << UNIVERSE_ID_SHIFT)
        | ((self.world_id    as UID) << WORLD_ID_SHIFT)
        | ((self.region_id   as UID) << REGION_ID_SHIFT)
        | ((self.id          as UID) << ID_SHIFT)
    }

    pub const fn into_uid(self) -> UID {
        self.to_uid()
    }
}

impl Into<UID> for Identity {
    fn into(self) -> UID {
        self.into_uid()
    }
}

impl From<UID> for Identity {
    fn from(value: UID) -> Self {
        Self::from_uid(value)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum IdentityField {
    ID,
    RegionID,
    WorldID,
    UniverseID,
}

impl Fields for IdentityField {
    fn field(&self) -> &'static Field {
        match self {
            Self::ID => &Self::FIELD_ID,
            Self::RegionID => &Self::FIELD_REGION_ID,
            Self::WorldID => &Self::FIELD_WORLD_ID,
            Self::UniverseID => &Self::FIELD_UNIVERSE_ID
        }
    }
}

impl IdentityField {
    const CLASSNAME: &'static str = "Identity";
    const FIELD_UNIVERSE_ID: Field = Field::new(Self::CLASSNAME, "universe_id", FieldValueType::UnsignedInteger);
    const FIELD_WORLD_ID: Field = Field::new(Self::CLASSNAME, "world_id", FieldValueType::UnsignedInteger);
    const FIELD_REGION_ID: Field = Field::new(Self::CLASSNAME, "region_id", FieldValueType::UnsignedInteger);
    const FIELD_ID: Field = Field::new(Self::CLASSNAME, "id", FieldValueType::UnsignedInteger);
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IdentityBuilder {
    builder_mode: BuilderMode,
    universe_id: Option<UniverseID>,
    world_id: Option<WorldID>,
    region_id: Option<RegionID>,
    id: Option<ID>
}

impl Builder for IdentityBuilder {
    type ModelType = Identity;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            universe_id: None,
            world_id: None,
            region_id: None,
            id: None,
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
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::ID.field().name()})?,
            region_id: self.region_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::RegionID.field().name()})?,
            world_id: self.world_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::WorldID.field().name()})?,
            universe_id: self.universe_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::UniverseID.field().name()})?
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

    pub fn all(&mut self, universe_id: UniverseID, world_id: WorldID, region_id: RegionID, id: ID) -> Result<()> {
        self.universe_id(universe_id)?;
        self.world_id(world_id)?;
        self.region_id(region_id)?;
        self.id(id)?;
        Ok(())
    }

    pub fn uid(&mut self, uid: UID) -> Result<()> {
        let identity = Identity::from_uid(uid);
        self.universe_id(identity.universe_id);
        self.world_id(identity.world_id);
        self.region_id(identity.region_id);
        self.id(identity.id);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uid_min_mid_max() {
        let expected: [(UniverseID, WorldID, RegionID, ID);4] = [
            (UniverseID::MIN, WorldID::MIN, RegionID::MIN, ID::MIN),
            (UniverseID::MIN + 1, WorldID::MIN + 1, RegionID::MIN + 1, ID::MIN + 1),
            (UniverseID::MAX / 2, WorldID::MAX / 2, RegionID::MAX / 2, ID::MAX / 2),
            (UniverseID::MAX, WorldID::MAX, RegionID::MAX, ID::MAX),
        ];

        for (universe_id, world_id, region_id, id) in expected {
            let identity = Identity::new(universe_id, world_id, region_id, id);
            let uid: UID = identity.into();
            let identity = Identity::from(uid);

            assert_eq!(universe_id, identity.universe_id());
            assert_eq!(world_id, identity.world_id());
            assert_eq!(region_id, identity.region_id());
            assert_eq!(id, identity.id());
        }
    }
}
