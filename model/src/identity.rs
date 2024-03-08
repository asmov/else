use serde;
use crate::{codebase::*, error::*, modeling::*};

pub type UID        = u128;
// UID is composed of:
pub type UniverseID = u16;
pub type WorldID    = u32;
pub type ClassID    = u16;
pub type ID         = u64;

const UNIVERSE_ID_SHIFT: usize = UID::BITS as usize - UniverseID::BITS as usize;
const WORLD_ID_SHIFT:    usize = UNIVERSE_ID_SHIFT  - WorldID::BITS as usize;
const CLASS_ID_SHIFT:    usize = WORLD_ID_SHIFT     - ClassID::BITS as usize;
const ID_SHIFT:          usize = CLASS_ID_SHIFT     - ID::BITS as usize;

#[derive(PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
pub struct Identity {
    universe_id: UniverseID,
    world_id: WorldID,
    class_id: ClassID,
    id: ID
}

pub trait MaybeIdentifiable {
    fn try_uid(&self) -> Result<UID>;
}

/// Unique to the World. Should be used to permanently reference objects (never use UID manually).
pub trait Keyed {
    fn key(&self) -> Option<&str> {
        None
    }
}
pub trait Identifiable: Keyed {
    fn uid(&self) -> UID;
}

impl MaybeIdentifiable for Identity {
    fn try_uid(&self) -> Result<UID> {
        Ok(self.into_uid())
    }
}

impl Keyed for Identity {}

impl Identifiable for Identity {
    fn uid(&self) -> UID {
        self.to_uid()
    }
}

impl MaybeIdentifiable for UID {
    fn try_uid(&self) -> Result<UID> {
        Ok(*self)
    }
}

impl Identifiable for UID {
    fn uid(&self) -> UID {
        *self
    }
}

impl Keyed for UID {}

impl Identity {
    pub fn new(universe_id: UniverseID, world_id: WorldID, class_id: ClassID, id: ID) -> Self {
        Self {
            universe_id,
            world_id,
            class_id,
            id
        }
    }

    pub const fn from_uid(value: UID) -> Self {
        Self {
            universe_id: (value >> UNIVERSE_ID_SHIFT) as UniverseID,
            world_id:    (value >> WORLD_ID_SHIFT)    as WorldID,
            class_id:    (value >> CLASS_ID_SHIFT)    as ClassID,
            id:          (value >> ID_SHIFT)          as ID 
        }
    }

    pub const fn to_uid(&self) -> UID {
        0
        | ((self.universe_id as UID) << UNIVERSE_ID_SHIFT)
        | ((self.world_id    as UID) << WORLD_ID_SHIFT)
        | ((self.class_id    as UID) << CLASS_ID_SHIFT)
        | ((self.id          as UID) << ID_SHIFT)
    }

    pub const fn into_uid(self) -> UID {
        self.to_uid()
    }

    pub fn split(self) -> (UniverseID, WorldID, ClassID, ID) {
        (self.universe_id, self.world_id, self.class_id, self.id)
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
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl IdentityField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Identity as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "Identity";
    const FIELD_UNIVERSE_ID: Field = Field::new(&Self::CLASS_IDENT, "universe_id", FieldValueType::U64);
    const FIELD_WORLD_ID: Field = Field::new(&Self::CLASS_IDENT, "world_id", FieldValueType::U64);
    const FIELD_CLASS_ID: Field = Field::new(&Self::CLASS_IDENT, "class_id", FieldValueType::U64);
    const FIELD_ID: Field = Field::new(&Self::CLASS_IDENT, "id", FieldValueType::U64);

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

pub trait BuildableUID: Builder + MaybeIdentifiable {
    fn uid(&mut self, uid: UID) -> Result<&mut Self>;
    fn get_uid(&self) -> Option<&UID>;

    fn uid_from(&mut self, identifiable: &impl Identifiable) -> Result<&mut Self> {
        self.uid(identifiable.uid())?;
        Ok(self)
    }

    fn has_uid(&self) -> bool {
        self.get_uid().is_some()
    }
    
    fn _try_uid(&self) -> Result<UID> {
        self.get_uid()
            .ok_or_else(|| Error::IdentityNotGenerated)
            .and_then(|uid| uid.try_uid())
    }
}

pub struct IdentityGenerator {
    universe_id: UniverseID,
    world_id: WorldID,
    next_id: ID
}

impl IdentityGenerator {
    pub fn new(universe_id: UniverseID, world_id: WorldID, next_id: ID) -> Self {
        Self {
            universe_id,
            world_id,
            next_id
        }
    }

    pub fn from_identity(identity: &Identity, next_id: ID) -> Self {
        Self {
            universe_id: identity.universe_id(),
            world_id: identity.world_id(),
            next_id
        }
    }

    pub fn from_uid(uid: UID, next_id: ID) -> Self {
        let identity = Identity::from_uid(uid);
        Self::from_identity(&identity, next_id)
    }

    pub fn next_uid(&mut self, class_id: ClassID) -> UID {
        let uid = Identity::new(self.universe_id, self.world_id, class_id, self.next_id).into_uid();
        self.next_id += 1;
        uid
    }

    pub fn next_id(&mut self) -> ID {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn next_identity(&mut self, class_id: ClassID) -> Identity {
        let identity = Identity::new(self.universe_id, self.world_id, class_id, self.next_id);
        self.next_id += 1;
        identity
    }

    pub fn get_next_id(&self) -> ID {
        self.next_id
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
