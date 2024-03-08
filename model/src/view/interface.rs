use serde;
use crate::{codebase::*, error::*, identity::*, interface::*, character::*, view::world::*, view::thing::*};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InterfaceView {
    interface: Interface,
    world_view: WorldView,
    downlink_uid: UID, // Thing
}

impl Keyed for InterfaceView {}

impl Identifiable for InterfaceView {
    fn uid(&self) -> UID {
        self.interface.uid()
    }
}

impl InterfaceView {
    pub fn interface(&self) -> &Interface {
        &self.interface
    }

    pub fn world_view(&self) -> &WorldView {
        &self.world_view
    }

    pub fn downlink_uid(&self) -> UID {
        self.downlink_uid
    }

    pub fn downlink(&self) -> &ThingView {
        &self.world_view.thing_view(self.downlink_uid).unwrap()
    }
}

pub enum InterfaceViewField {
    Interface,
    World,
    Downlink
}

impl Fields for InterfaceViewField {
    fn field(&self) -> &'static Field {
        match self {
            Self::Interface => &Self::FIELD_INTERFACE,
            Self::World => &Self::FIELD_WORLD,
            Self::Downlink => &Self::FIELD_WORLD,
        }
    }
}

impl Class for InterfaceViewField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl InterfaceViewField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::InterfaceView as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "InterfaceView";
    const FIELDNAME_INTERFACE: &'static str = "interface";
    const FIELDNAME_WORLD: &'static str = "world";
    const FIELDNAME_DOWNLINK: &'static str = "downlink";

    const FIELD_INTERFACE: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_INTERFACE,
        FieldValueType::Model(InterfaceField::class_ident_const()));
    const FIELD_WORLD: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_WORLD,
        FieldValueType::Model(WorldViewField::class_ident_const()));
    const FIELD_DOWNLINK: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_WORLD,
        FieldValueType::UID(WorldViewField::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    } 
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InterfaceViewBuilder {
    builder_mode: BuilderMode,
    interface: Option<InterfaceBuilder>,
    world_view: Option<WorldViewBuilder>,
    downlink_uid: Option<UID>
}

impl Builder for InterfaceViewBuilder {
    type BuilderType = Self;
    type ModelType = InterfaceView;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            interface: None,
            world_view: None,
            downlink_uid: None
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
        InterfaceViewField::class_ident_const()
    }

    fn create(mut self) -> Result<Creation<Self::BuilderType>> {
        let mut fields_changed = FieldsChanged::from_builder(&self);

        let interface = Build::create(&mut self.interface, &mut fields_changed, InterfaceViewField::Interface)?;
        let world_view = Build::create(&mut self.world_view, &mut fields_changed, InterfaceViewField::World)?;
        let downlink_uid = Build::create_uid(&mut self.downlink_uid, &mut fields_changed, InterfaceViewField::Downlink)?;

        let interface_view = InterfaceView {
            interface,
            world_view,
            downlink_uid
        };

        Ok(Creation::new(self, interface_view))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Build::prepare_modify_composite(&mut self, existing)?;

        Build::modify(&mut self.interface, &mut existing.interface, &mut fields_changed, InterfaceViewField::Interface)?;
        Build::modify(&mut self.world_view, &mut existing.world_view, &mut fields_changed, InterfaceViewField::World)?;

        Ok(Modification::new(self, fields_changed))
    }
}

impl SynchronizedDomainBuilder<InterfaceView> for InterfaceViewBuilder {
    fn synchronize(self, interface_view: &mut InterfaceView) -> Result<Modification<Self::BuilderType>> {
        self.modify(interface_view)
    }
}

impl InterfaceViewBuilder {
    pub fn interface(&mut self, interface: InterfaceBuilder) -> Result<&mut Self> {
        self.interface = Some(interface);
        Ok(self)
    }

    pub fn world_view(&mut self, world_view: WorldViewBuilder) -> Result<&mut Self> {
        self.world_view = Some(world_view);
        Ok(self)
    }

    pub fn downlink_uid(&mut self, downlink_uid: UID) -> Result<&mut Self> {
        self.downlink_uid = Some(downlink_uid);
        Ok(self)
    }
}