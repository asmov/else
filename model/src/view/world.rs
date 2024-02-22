use crate::{error::*, modeling::*, codebase::*, identity::*, route::*, timeframe::*, view::area::*, world::*};

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
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl WorldViewField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::WorldView as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "WorldView";
    const FIELD_UID: Field = Field::new(&Self::CLASS_IDENT, "uid", FieldValueType::UID);
    const FIELD_FRAME: Field = Field::new(&Self::CLASS_IDENT, "frame", FieldValueType::U64);
    const FIELD_AREA: Field = Field::new(&Self::CLASS_IDENT, "area", FieldValueType::Model(AreaViewField::class_ident_const()));
    const FIELD_ROUTES: Field = Field::new(&Self::CLASS_IDENT, "routes", FieldValueType::UIDList);
}

pub struct WorldViewBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    frame: Option<Frame>,
    area_view: Option<AreaViewBuilder>,
    routes: Vec<ListOp<RouteBuilder, UID>>
}

impl Builder for WorldViewBuilder {
    type DomainType = World;
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
        let mut fields_changed = FieldsChanged::from_builder(&self);

        let uid = Build::create(&mut self.identity, &mut fields_changed, WorldViewField::UID)?.to_uid();
        let frame = Build::create_value(&self.frame, &mut fields_changed, WorldViewField::Frame)?;
        let area_view = Build::create(&mut self.area_view, &mut fields_changed, WorldViewField::Area)?;
        let routes = Build::create_vec(&mut self.routes, &mut fields_changed, WorldViewField::Routes)?;

        let world_view = WorldView {
            uid,
            frame,
            area_view,
            routes
        };

        Ok(Creation::new(self, world_view))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = FieldsChanged::from_builder(&self);

        if self.frame.is_some() {
            Build::modify_value(&mut self.frame, &mut fields_changed, WorldField::Frame)?;
        }
        if self.area_view.is_some() {
            Build::modify(&mut self.area_view, &mut existing.area_view, &mut fields_changed, WorldField::Areas)?;
        }
        
        Build::modify_vec(&mut self.routes, &mut existing.routes, &mut fields_changed, WorldViewField::Routes)?;

        Ok(Modification::new(self, fields_changed))
    }

    fn class_ident(&self) -> &'static ClassIdent {
        WorldViewField::class_ident()
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
        self.routes.push(ListOp::Add(route));
        Ok(self)
    }
}

