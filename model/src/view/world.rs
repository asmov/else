use crate::{error::*, modeling::*, codebase::*, identity::*, route::*, timeframe::*, view::area::*, view::thing::*, world::*, sync::*};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WorldView {
    uid: UID,
    frame: Frame,
    area_view: AreaView,
    thing_views: Vec<ThingView>,
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

    pub fn route(&self, uid: UID) -> Result<&Route> {
        self.routes.iter()
            .find(|route| route.uid() == uid)
            .ok_or_else(|| Error::ModelNotFound { model: RouteField::classname(), uid })
    }

    pub fn thing_views(&self) -> &Vec<ThingView> {
        &self.thing_views
    }

    pub fn thing_view(&self, uid: UID) -> Result<&ThingView> {
        self.thing_views.iter()
            .find(|thing_view| thing_view.uid() == uid)
            .ok_or_else(|| Error::ModelNotFound { model: ThingViewField::classname(), uid })
    }
}

pub enum WorldViewField {
    UID,
    Frame,
    Area,
    Routes,
    Things
}

impl Fields for WorldViewField {
    fn field(&self) -> &'static Field {
        match self {
            Self::UID => &Self::FIELD_UID,
            Self::Frame => &Self::FIELD_FRAME,
            Self::Area => &Self::FIELD_AREA,
            Self::Routes => &Self::FIELD_ROUTES,
            Self::Things => &Self::FIELD_THINGS
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
    const FIELD_UID: Field = Field::new(&Self::CLASS_IDENT, "uid", FieldValueType::UID(&Self::CLASS_IDENT));
    const FIELD_FRAME: Field = Field::new(&Self::CLASS_IDENT, "frame", FieldValueType::U64);
    const FIELD_AREA: Field = Field::new(&Self::CLASS_IDENT, "area", FieldValueType::Model(AreaViewField::class_ident_const()));
    const FIELD_ROUTES: Field = Field::new(&Self::CLASS_IDENT, "routes", FieldValueType::Model(RouteField::class_ident_const()));
    const FIELD_THINGS: Field = Field::new(&Self::CLASS_IDENT, "things", FieldValueType::Model(ThingViewField::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WorldViewBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    frame: Option<Frame>,
    area_view: Option<AreaViewBuilder>,
    routes: Vec<ListOp<RouteBuilder, UID>>,
    thing_views: Vec<ListOp<ThingViewBuilder, UID>>
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
            routes: Vec::new(),
            thing_views: Vec::new()
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
        let thing_views = Build::create_vec(&mut self.thing_views, &mut fields_changed, WorldViewField::Things)?;

        let world_view = WorldView {
            uid,
            frame,
            area_view,
            routes,
            thing_views
        };

        Ok(Creation::new(self, world_view))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Build::prepare_modify_composite(&mut self, existing)?;

        Build::modify_value(&mut self.frame, &mut existing.frame, &mut fields_changed, WorldField::Frame)?;
        Build::modify(&mut self.area_view, &mut existing.area_view, &mut fields_changed, WorldField::Areas)?;
        Build::modify_vec(&mut self.routes, &mut existing.routes, &mut fields_changed, WorldViewField::Routes)?;
        Build::modify_vec(&mut self.thing_views, &mut existing.thing_views, &mut fields_changed, WorldViewField::Things)?;

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

    pub fn add_thing_view(&mut self, thing_view: ThingViewBuilder) -> Result<&mut Self> {
        self.thing_views.push(ListOp::Add(thing_view));
        Ok(self)
    }
}

