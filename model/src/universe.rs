use serde;
use crate::{error::*, codebase::*, identity::*, session::*, descriptor::*, modeling::*, world::*, interface::*};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Universe {
    uid: UID,
    descriptor: Descriptor,
    world_uids: Vec<UID>,
    sessions: Vec<Session>
}

impl Universe {
    pub fn world_uids(&self) -> &Vec<UID> {
        &self.world_uids
    }

    pub fn sessions(&self) -> &Vec<Session> {
        &self.sessions
    }
}

impl Keyed for Universe {
    fn key(&self) -> Option<&str> {
        self.descriptor.key()
    }
}

impl Identifiable for Universe {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl Descriptive for Universe {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl Built for Universe {
    type BuilderType = UniverseBuilder;
}

pub enum UniverseField {
    UID,
    Descriptor,
    WorldUIDs,
    Sessions
}

impl Fields for UniverseField {
    fn field(&self) -> &'static Field {
        match self {
            Self::UID => &Self::FIELD_UID,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::WorldUIDs => &Self::FIELD_WORLD_UIDS,
            Self::Sessions => &Self::FIELD_SESSIONS
        }
    }
}

impl Class for UniverseField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl UniverseField {
    const CLASSNAME: &'static str = "Universe";
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Universe as ClassID, Self::CLASSNAME);
    const FIELDNAME_UID: &'static str = "uid";
    const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    const FIELDNAME_WORLD_UIDS: &'static str = "world_uids";
    const SESSIONS: &'static str = "active_interface_uids";
    const FIELD_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_UID,
        FieldValueType::UID(&Self::CLASS_IDENT));
    const FIELD_DESCRIPTOR: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_DESCRIPTOR,
        FieldValueType::Model(DescriptorField::class_ident_const()));
    const FIELD_WORLD_UIDS: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_WORLD_UIDS,
        FieldValueType::UIDList(WorldField::class_ident_const()));
    const FIELD_SESSIONS: Field = Field::new(&Self::CLASS_IDENT, Self::SESSIONS,
        FieldValueType::UIDList(InterfaceField::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

pub struct UniverseBuilder {
    builder_mode: BuilderMode,
    uid: Option<UID>,
    descriptor: Option<DescriptorBuilder>,
    world_uids: Vec<ListOp<UID, UID>>,
    sessions: Vec<ListOp<SessionBuilder, UID>>
}

impl Builder for UniverseBuilder {
    type ModelType = Universe;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            uid: None,
            descriptor: None,
            world_uids: Vec::new(),
            sessions: Vec::new()
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

    fn class_ident(&self) -> &'static ClassIdent {
        &UniverseField::CLASS_IDENT
    }

    fn create(mut self) -> crate::Result<Creation<Self::BuilderType>> {
        let mut fields_changed = FieldsChanged::from_builder(&self);

        let uid = Build::create_uid(&mut self.uid, &mut fields_changed, UniverseField::UID)?;
        let descriptor = Build::create(&mut self.descriptor, &mut fields_changed, UniverseField::Descriptor)?;
        let world_uids = Build::create_uid_vec(&mut self.world_uids, &mut fields_changed, UniverseField::WorldUIDs)?;
        let sessions = Build::create_vec(&mut self.sessions, &mut fields_changed, UniverseField::Sessions)?;

        let universe = Universe {
            uid,
            descriptor,
            world_uids,
            sessions
        };

        Ok(Creation::new(self, universe))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> crate::Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Build::prepare_modify(&mut self, existing)?;

        Build::modify(&mut self.descriptor, &mut existing.descriptor, &mut fields_changed, UniverseField::Descriptor)?;
        Build::modify_uid_vec(&mut self.world_uids, &mut existing.world_uids, &mut fields_changed, UniverseField::WorldUIDs)?;
        Build::modify_vec(&mut self.sessions, &mut existing.sessions, &mut fields_changed, UniverseField::Sessions)?;

        Ok(Modification::new(self, fields_changed))
    }
}

impl MaybeIdentifiable for UniverseBuilder {
    fn try_uid(&self) -> Result<UID> {
        Self::_try_uid(&self)
    }
}

impl BuildableUID for UniverseBuilder {
    fn uid(&mut self, uid: UID) -> Result<&mut Self> {
        self.uid = Some(uid);
        Ok(self)
    }

    fn get_uid(&self) -> Option<&UID> {
        self.uid.as_ref()
    }
}

impl BuildableDescriptor for UniverseBuilder {
    fn descriptor(&mut self, descriptor: DescriptorBuilder) -> Result<&mut Self> {
        self.descriptor = Some(descriptor);
        Ok(self)
    }
    
    fn descriptor_builder(&mut self) -> &mut DescriptorBuilder {
        if self.descriptor.is_none() {
            self.descriptor = Some(Descriptor::creator());
        }

        self.descriptor.as_mut().unwrap()
    }
}

impl UniverseBuilder {
    pub fn add_world_uid(&mut self, uid: UID) -> Result<&mut Self> {
        self.world_uids.push(ListOp::Add(uid));
        Ok(self)
    }

    pub fn remove_world_uid(&mut self, uid: UID) -> Result<&mut Self> {
        self.world_uids.push(ListOp::Remove(uid));
        Ok(self)
    }
}

impl UniverseBuilder {
    pub fn add_session(&mut self, session: SessionBuilder) -> Result<&mut Self> {
        self.sessions.push(ListOp::Add(session));
        Ok(self)
    }

    pub fn remove_session(&mut self, uid: UID) -> Result<&mut Self> {
        self.sessions.push(ListOp::Remove(uid));
        Ok(self)
    }
}

