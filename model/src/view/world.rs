use crate::{error::*, builder::*, classes::*, identity::*, route::*, timeframe::*, view::area::*};

pub struct WorldView {
    uid: UID,
    frame: Frame,
    area_view: AreaView,
    routes: Vec<Route>
}

impl Keyed for WorldView {}

impl Identifiable for WorldView {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl Built for WorldView {
    type BuilderType = WorldViewBuilder;
}

impl WorldView {
    pub fn frame(&self) -> Frame {
        self.frame
    }

    pub fn area_view(&self) -> &AreaView {
        &self.area_view
    }

    pub fn routes(&self) -> &Vec<Route> {
        &self.routes
    }
}

pub enum WorldViewField {
    UID,
    Frame,
    Area,
    Routes
}

impl Fields for WorldViewField {
    fn field(&self) -> &'static Field {
        match self {
            Self::UID => &Self::FIELD_UID,
            Self::Frame => &Self::FIELD_FRAME,
            Self::Area => &Self::FIELD_AREA,
            Self::Routes => &Self::FIELD_ROUTES
        }
    }
}

impl Class for WorldViewField {
    fn class_id() -> ClassID {
        Self::CLASS_ID
    }

    fn classname() -> &'static str {
        Self::CLASSNAME
    }
}

impl WorldViewField {
    const CLASS_ID: ClassID = ClassIdent::WorldView as ClassID;
    const CLASSNAME: &'static str = "WorldView";
    const FIELD_UID: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, "uid", FieldValueType::UID);
    const FIELD_FRAME: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, "frame", FieldValueType::U64);
    const FIELD_AREA: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, "area", FieldValueType::Model);
    const FIELD_ROUTES: Field = Field::new(Self::CLASS_ID, Self::CLASSNAME, "routes", FieldValueType::VecUID);
}

pub struct WorldViewBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    frame: Option<Frame>,
    area_view: Option<AreaViewBuilder>,
    routes: Vec<RouteBuilder>
}

impl Builder for WorldViewBuilder {
    type ModelType = WorldView;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            identity: None,
            frame: None,
            area_view: None,
            routes: Vec::new()
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
        let uid = Creation::try_assign(&mut self.identity, WorldViewField::UID)?.to_uid();
        let frame = Self::try_assign_value(&mut self.frame, WorldViewField::Frame)?;
        let area_view = Creation::try_assign(&mut self.area_view, WorldViewField::Area)?;
        let routes = Creation::assign_vec(&mut self.routes)?;

        let world_view = WorldView {
            uid,
            frame,
            area_view,
            routes
        };

        Ok(Creation::new(self, world_view))
    }

    fn modify(mut self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Vec::new();

        if self.frame.is_some() {
            original.frame = self.frame.unwrap();
            fields_changed.push(WorldViewField::Frame.field());
        }
        if self.area_view.is_some() {
            Modification::assign(&mut self.area_view, &mut original.area_view)?;
            fields_changed.push(WorldViewField::Area.field());
        }
        
        Creation::modify_vec(&mut self.routes, &mut original.routes)?;

        Ok(Modification::new(self, fields_changed))
    }

    fn class_id(&self) -> ClassID {
        WorldViewField::class_id()
    }
}

impl WorldViewBuilder {
    pub fn identity(&mut self, identity: IdentityBuilder) -> Result<&mut Self> {
        self.identity = Some(identity);
        Ok(self) 
    }

    pub fn frame(&mut self, frame: Frame) -> Result<&mut Self> {
        self.frame = Some(frame);
        Ok(self)
    }

    pub fn area_view(&mut self, area_view: AreaViewBuilder) -> Result<&mut Self> {
        self.area_view = Some(area_view);
        Ok(self)
    }

    pub fn add_route(&mut self, route: RouteBuilder) -> Result<&mut Self> {
        self.routes.push(route);
        Ok(self)
    }
}

