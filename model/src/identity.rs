use serde;
use crate::{classes::*, error::*, builder::*};

pub type UID        = u128;
// UID is composed of:
pub type UniverseID = u16;
pub type WorldID    = u32;
pub type ClassID    = u16;
pub type ID         = u64;

const UID_BITS:         usize = std::mem::size_of::<UID>()         * 8;
const UNIVERSE_ID_BITS: usize = std::mem::size_of::<UniverseID>()  * 8;
const WORLD_ID_BITS:    usize = std::mem::size_of::<WorldID>()     * 8;
const CLASS_ID_BITS:    usize = std::mem::size_of::<ClassID>()    * 8;
const ID_BITS:          usize = std::mem::size_of::<ID>()          * 8;

const UNIVERSE_ID_SHIFT:    usize = UID_BITS          - UNIVERSE_ID_BITS;
const WORLD_ID_SHIFT:       usize = UNIVERSE_ID_SHIFT - WORLD_ID_BITS;
const CLASS_ID_SHIFT:       usize = WORLD_ID_SHIFT    - CLASS_ID_BITS;
const ID_SHIFT:             usize = CLASS_ID_SHIFT    - ID_BITS;

#[derive(PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
pub struct Identity {
    universe_id: UniverseID,
    world_id: WorldID,
    class_id: ClassID,
    id: ID
}

pub trait Identifiable {
    fn uid(&self) -> UID;
}

impl Built for Identity {
    type BuilderType = IdentityBuilder;
}

impl Identity {
    pub fn new(universe_id: UniverseID, world_id: WorldID, region_id: ClassID, id: ID) -> Self {
        Self {
            universe_id,
            world_id,
            class_id: region_id,
            id
        }
    }

    pub const fn from_uid(value: UID) -> Self {
        Self {
            universe_id:    (value >> UNIVERSE_ID_SHIFT)    as UniverseID,
            world_id:       (value >> WORLD_ID_SHIFT)       as WorldID,
            class_id:      (value >> CLASS_ID_SHIFT)      as ClassID,
            id:             (value >> ID_SHIFT)             as ID 
        }
    }

    pub const fn to_uid(&self) -> UID {
        0
        | ((self.universe_id as UID) << UNIVERSE_ID_SHIFT)
        | ((self.world_id    as UID) << WORLD_ID_SHIFT)
        | ((self.class_id   as UID) << CLASS_ID_SHIFT)
        | ((self.id          as UID) << ID_SHIFT)
    }

    pub const fn into_uid(self) -> UID {
        self.to_uid()
    }

    pub fn id_to_string(&self) -> String {
        let mut chars: Vec<char> = Vec::new();
        let mut x = self.id();
        loop {
            let m = (x % RADIX as ID) as usize;
            x = x / RADIX as ID;

            chars.push(CHARMAP[m]);

            if x == 0 {
                break;
            }
        }

        chars.into_iter().collect()
    }
}

const RADIX: usize = 62;
const CHARMAP: [char; RADIX] = [
    '0',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9',
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g',
    'h',
    'i',
    'j',
    'k',
    'l',
    'm',
    'n',
    'o',
    'p',
    'q',
    'r',
    's',
    't',
    'u',
    'v',
    'w',
    'x',
    'y',
    'z',
    'A',
    'B',
    'C',
    'D',
    'E',
    'F',
    'G',
    'H',
    'I',
    'J',
    'K',
    'L',
    'M',
    'N',
    'O',
    'P',
    'Q',
    'R',
    'S',
    'T',
    'U',
    'V',
    'W',
    'X',
    'Y',
    'Z',
];


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

pub trait IntoIdentity {
    fn into_identity(self) -> Identity;
}

impl IntoIdentity for UID {
    fn into_identity(self) -> Identity {
        self.into()
    }
}

impl Identity {
    pub fn universe_id(&self) -> UniverseID {
        self.universe_id
    }

    pub fn world_id(&self) -> WorldID {
        self.world_id
    }

    pub fn class_id(&self) -> ClassID {
        self.class_id
    }

    pub fn id(&self) -> ID {
        self.id
    }
}

#[derive(Clone, Copy, Debug)]
pub enum IdentityField {
    UniverseID,
    WorldID,
    ClassID,
    ID
}

impl Fields for IdentityField {
    fn field(&self) -> &'static Field {
        match self {
            Self::UniverseID => &Self::FIELD_UNIVERSE_ID,
            Self::WorldID => &Self::FIELD_WORLD_ID,
            Self::ClassID => &Self::FIELD_CLASS_ID,
            Self::ID => &Self::FIELD_ID,
        }
    }
}

impl Class for IdentityField {
    fn class_id() -> ClassID {
        Self::CLASS_ID
    }

    fn classname() -> &'static str {
        Self::CLASSNAME
    }
}

impl IdentityField {
    const CLASS_ID: ClassID = ClassIdent::Identity as ClassID;
    const CLASSNAME: &'static str = "Identity";
    const FIELD_UNIVERSE_ID: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, "universe_id", FieldValueType::U64);
    const FIELD_WORLD_ID: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, "world_id", FieldValueType::U64);
    const FIELD_CLASS_ID: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, "class_id", FieldValueType::U64);
    const FIELD_ID: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, "id", FieldValueType::U64);
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IdentityBuilder {
    builder_mode: BuilderMode,
    universe_id: Option<UniverseID>,
    world_id: Option<WorldID>,
    class_id: Option<ClassID>,
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
            class_id: None,
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
            class_id: self.class_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::ClassID.field().name()})?,
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
        if let Some(region_id) = self.class_id {
            original.class_id = region_id;
            fields_changed.push(IdentityField::ClassID.field());
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

    fn class_id(&self) -> ClassID {
        IdentityField::class_id()
    }
}

impl IdentityBuilder {
    pub fn universe_id(&mut self, universe_id: UniverseID) -> Result<&mut Self> {
        self.universe_id = Some(universe_id);
        Ok(self)
    }

    pub fn world_id(&mut self, world_id: WorldID) -> Result<&mut Self> {
        self.world_id = Some(world_id);
        Ok(self)
    }

    pub fn class_id(&mut self, region_id: ClassID) -> Result<&mut Self> {
        self.class_id = Some(region_id);
        Ok(self)
    }

    pub fn id(&mut self, id: ID) -> Result<&mut Self> {
        self.id = Some(id);
        Ok(self)
    }
    
    pub fn uid(&mut self, uid: UID) -> Result<()> {
        let identity = Identity::from_uid(uid);
        self.universe_id(identity.universe_id)?;
        self.world_id(identity.world_id)?;
        self.class_id(identity.class_id)?;
        self.id(identity.id)?;
        Ok(())
    }

    pub fn get_universe_id(&self) -> Option<UniverseID> {
        self.universe_id
    }

    pub fn get_world_id(&self) -> Option<WorldID> {
        self.world_id
    }

    pub fn get_class_id(&self) -> Option<ClassID> {
        self.class_id
    }

    pub fn get_id(&self) -> Option<ID> {
        self.id
    }

    pub fn get_uid(&self) -> Result<UID> {
        let identity = Identity {
            id: self.id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::ID.field().name()})?,
            class_id: self.class_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::ClassID.field().name()})?,
            world_id: self.world_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::WorldID.field().name()})?,
            universe_id: self.universe_id.ok_or_else(||
                Error::FieldNotSet {class: IdentityField::CLASSNAME, field: IdentityField::UniverseID.field().name()})?
        };

        Ok(identity.into())
    }

    pub fn from_original(builder: &impl Builder, identifiable: &impl Identifiable) -> Self {
        let mut me = Self::builder(builder.builder_mode());
        me.uid(identifiable.uid()).unwrap();
        me
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
        let expected: [(UniverseID, WorldID, ClassID, ID);4] = [
            (UniverseID::MIN, WorldID::MIN, ClassID::MIN, ID::MIN),
            (UniverseID::MIN + 1, WorldID::MIN + 1, ClassID::MIN + 1, ID::MIN + 1),
            (UniverseID::MAX / 2, WorldID::MAX / 2, ClassID::MAX / 2, ID::MAX / 2),
            (UniverseID::MAX, WorldID::MAX, ClassID::MAX, ID::MAX),
        ];

        for (universe_id, world_id, class_id, id) in expected {
            let identity = Identity::new(universe_id, world_id, class_id, id);
            let uid: UID = identity.into();
            let identity = Identity::from(uid);

            assert_eq!(universe_id, identity.universe_id());
            assert_eq!(world_id, identity.world_id());
            assert_eq!(class_id, identity.class_id());
            assert_eq!(id, identity.id());
        }
    }
}
