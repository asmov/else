pub mod direction;
pub mod end;
pub mod endpoint;
pub mod junction;
pub mod point;

use crate::{codebase::*, descriptor::*, error::*, identity::*, modeling::*, world::*, AreaField};
use serde;

pub use crate::route::{end::*, endpoint::*, junction::*, point::*, direction::*};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Route {
    uid: UID,
    descriptor: Descriptor,
    point_a: Point,
    point_b: Point 
}

impl Keyed for Route {
    fn key(&self) -> Option<&str> {
        self.descriptor.key()
    }
}

impl Identifiable for Route {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl Descriptive for Route {
    fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}

impl Built for Route {
    type BuilderType = RouteBuilder;
}

impl Route {
    pub fn point_a(&self) -> &Point {
        &self.point_a
    }

    pub fn point_b(&self) -> &Point {
        &self.point_b
    }

    pub fn area_uids(&self) -> Vec<UID> {
        let mut uids = self.point_a.area_uids();
        uids.extend(self.point_b.area_uids());
        uids
    }

    pub fn end_for_area(&self, area_uid: UID) -> Option<&End> {
        match self.point_a.end_for_area(area_uid) {
            Some(end) => Some(end),
            None => {
               match self.point_b.end_for_area(area_uid) {
                   Some(end) => Some(end),
                   None => None
               } 
            }
        }
    }
}

pub trait Routing {
    fn route_uids(&self) -> &Vec<UID>;
}

#[derive(Clone, Copy, Debug)]
pub enum RouteField {
    UID,
    Descriptor,
    PointA,
    PointB
}

impl Fields for RouteField {
    fn field(&self) -> &'static Field {
        match self {
            Self::UID => &Self::FIELD_UID,
            Self::Descriptor => &Self::FIELD_DESCRIPTOR,
            Self::PointA => &Self::FIELD_POINT_A,
            Self::PointB => &Self::FIELD_POINT_B
        }
    }
}

impl Class for RouteField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl RouteField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Route as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "Route";
    const FIELDNAME_UID: &'static str = "uid";
    const FIELDNAME_DESCRIPTOR: &'static str = "descriptor";
    const FIELDNAME_POINT_A: &'static str = "point_a";
    const FIELDNAME_POINT_B: &'static str = "point_b";

    const FIELD_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_UID, FieldValueType::Model(IdentityField::class_ident_const()));
    const FIELD_DESCRIPTOR: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_DESCRIPTOR, FieldValueType::Model(DescriptorField::class_ident_const()));
    const FIELD_POINT_A: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_POINT_A, FieldValueType::Model(EndpointField::class_ident_const()));
    const FIELD_POINT_B: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_POINT_B, FieldValueType::Model(EndpointField::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RouteBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>,
    descriptor: Option<DescriptorBuilder>,
    point_a: Option<PointBuilder>,
    point_b: Option<PointBuilder>
}

impl Builder for RouteBuilder {
    type ModelType = Route;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            identity: None,
            descriptor: None,
            point_a: None,
            point_b: None
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

        let uid = Build::create(&mut self.identity, &mut fields_changed, RouteField::UID)?.uid();
        let descriptor = Build::create(&mut self.descriptor, &mut fields_changed, RouteField::Descriptor)?;
        let point_a = Build::create(&mut self.point_a, &mut fields_changed, RouteField::PointA)?;
        let point_b = Build::create(&mut self.point_b, &mut fields_changed, RouteField::PointB)?;

        let route = Route {
            uid,
            descriptor,
            point_a,
            point_b,
        };

        Ok(Creation::new(self, route))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self>> {
        let mut fields_changed = Build::prepare_modify(&mut self, existing)?;

        Build::modify(&mut self.descriptor, &mut existing.descriptor, &mut fields_changed, RouteField::Descriptor)?;
        Build::modify(&mut self.point_a, &mut existing.point_a, &mut fields_changed, RouteField::PointA)?;
        Build::modify(&mut self.point_b, &mut existing.point_b, &mut fields_changed, RouteField::PointB)?;

        Ok(Modification::new(self, fields_changed))
    }

    fn class_ident(&self) -> &'static ClassIdent {
        RouteField::class_ident()
    }
}

impl MaybeIdentifiable for RouteBuilder {
    fn try_uid(&self) -> Result<UID> {
        Self::_try_uid(&self)
    }
}

impl BuildableIdentity for RouteBuilder {
    fn identity(&mut self, id: IdentityBuilder) -> Result<&mut Self> {
        self.identity = Some(id);
        Ok(self)
    }

    fn identity_builder(&mut self) -> &mut IdentityBuilder {
        if self.identity.is_none() {
            self.identity = Some(Identity::builder(self.builder_mode()));
        }

        self.identity.as_mut().unwrap()
    }

    fn get_identity(&self) -> Option<&IdentityBuilder> {
        self.identity.as_ref()
    }
}

impl BuildableDescriptor for RouteBuilder {
    fn descriptor(&mut self, descriptor: DescriptorBuilder) -> Result<&mut Self> {
        self.descriptor = Some(descriptor);
        Ok(self)
    }

    fn descriptor_builder(&mut self) -> &mut DescriptorBuilder {
        if self.descriptor.is_none() {
            self.descriptor = Some(Descriptor::builder(self.builder_mode()));
        }

        self.descriptor.as_mut().unwrap()
    }
}

impl RouteBuilder {
    pub fn point_a(&mut self, point_a: PointBuilder) -> Result<&mut Self> {
        self.point_a = Some(point_a);
        Ok(self)
    }

    pub fn point_a_builder(&mut self) -> &mut PointBuilder {
        todo!()
    }

    pub fn point_b(&mut self, point_b: PointBuilder) -> Result<&mut Self> {
        assert!(matches!(point_b, PointBuilder::Endpoint(_)));
        self.point_b = Some(point_b);
        Ok(self)
    }

    pub fn point_b_builder(&mut self) -> &mut PointBuilder {
        todo!()
    }

    pub fn creation_area_uids(&self) -> Result<Vec<UID>> {
        let mut uids = Vec::new();
        uids.extend(self.point_a.as_ref().unwrap().area_uids()?);
        uids.extend(self.point_b.as_ref().unwrap().area_uids()?);
        Ok(uids)
    }

    pub fn modification_area_uids(&self, existing: &Route) -> Result<Vec<UID>> {
        let mut uids = Vec::new();

        if let Some(point_a) = &self.point_a {
            uids.extend(point_a.area_uids()?);
        } else {
            uids.extend(existing.point_a.area_uids());
        }

        if let Some(point_b) = &self.point_b {
            uids.extend(point_b.area_uids()?);
        } else {
            uids.extend(existing.point_b.area_uids());
        }

        Ok(uids)
    }
}

pub trait BuildableRouteVector {
    fn add_route(&mut self, route: RouteBuilder) -> Result<&mut Self>; 
    fn edit_route(&mut self, route: RouteBuilder) -> Result<&mut Self>; 
    fn remove_route(&mut self, route_uid: UID) -> Result<&mut Self>; 
}

pub trait BuildableRouteUIDList {
    fn add_route_uid(&mut self, uid: UID) -> Result<&mut Self>; 
    fn remove_route_uid(&mut self, uid: UID) -> Result<&mut Self>; 
}

impl CloneBuilding for RouteBuilder {
    fn clone_model(builder_mode: BuilderMode, existing: &Route) -> Self {
        Self {
            builder_mode,
            identity: Some(IdentityBuilder::clone_uid(builder_mode, existing.uid)),
            descriptor: Some(DescriptorBuilder::clone_model(builder_mode, &existing.descriptor)),
            point_a: Some(PointBuilder::clone_model(builder_mode, &existing.point_a)),
            point_b: Some(PointBuilder::clone_model(builder_mode, &existing.point_b))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const CAT_TO_DOG_HOUSE: &str = "cat_house_to_dog_house";

    fn create_route(mut world: World) -> World {
        let cat_house = world.find_area(testing::CAT_HOUSE).unwrap(); // Point A
        let dog_house = world.find_area(testing::DOG_HOUSE).unwrap(); // Point B
        
        // route 'cat_house' to 'dog_house'
        let mut world_editor = World::editor();
        let mut route_cat_to_dog_house = Route::creator();
        route_cat_to_dog_house
            .point_a({
                let mut point_a = Endpoint::creator();
                point_a.end({ 
                    let mut end = End::creator();
                    end
                        .area_identity(IdentityBuilder::from_existing(&point_a, cat_house)).unwrap()
                        .direction(Direction::up()).unwrap()
                        .descriptor({
                            let mut descriptor = Descriptor::creator();
                            descriptor
                                .name(s!("portal_to_cat_house")).unwrap()
                                .description(s!("A portal swirling above")).unwrap();
                            descriptor
                        }).unwrap();
                    end
                }).unwrap();
                point_a.point_builder()
            }).unwrap()
            .point_b({
                let mut point_b = Endpoint::creator();
                point_b.end({ 
                    let mut end = End::creator();
                    end
                        .area_identity(IdentityBuilder::from_existing(&point_b, dog_house)).unwrap()
                        .direction(Direction::up()).unwrap()
                        .descriptor({
                            let mut descriptor = Descriptor::creator();
                            descriptor
                                .name(s!("portal_to_dog_house")).unwrap()
                                .description(s!("A portal swirling above")).unwrap();
                            descriptor
                        }).unwrap();
                    end
                }).unwrap();
                point_b.point_builder()
            }).unwrap()
            .descriptor({
                let mut descriptor = Descriptor::creator();
                descriptor
                    .key(CAT_TO_DOG_HOUSE.to_string()).unwrap()
                    .name(s!("Portal between Cat House and Dog House")).unwrap()
                    .description(s!("A route from the cat house to the dog house")).unwrap();
                descriptor
            }).unwrap();
        world_editor.add_route(route_cat_to_dog_house).unwrap();
        world_editor.modify(&mut world).unwrap();

        world
    }

    #[test]
    pub fn test_create_route() {
        let mut world = testing::create_world();
        world = create_route(world);

        // assertions
        let route_cat_to_dog_house = world.find_route(CAT_TO_DOG_HOUSE).unwrap();
        let cat_house = world.find_area(testing::CAT_HOUSE).unwrap(); // Point A
        let dog_house = world.find_area(testing::DOG_HOUSE).unwrap(); // Point B

        assert!(cat_house.route_uids().contains(&route_cat_to_dog_house.uid()));
        assert!(dog_house.route_uids().contains(&route_cat_to_dog_house.uid()));

        if let Point::Endpoint(endpoint) = route_cat_to_dog_house.point_a() {
            assert_eq!(endpoint.end().area_uid(), cat_house.uid());
        } else {
            panic!("Route Point A should be an Endpoint");
        }

        if let Point::Endpoint(endpoint) = route_cat_to_dog_house.point_b() {
            assert_eq!(endpoint.end().area_uid(), dog_house.uid());
        } else {
            panic!("Route Point B should be an Endpoint");
        }
    }

    #[test]
    pub fn test_edit_route() {
        let mut world = testing::create_world();
        world = create_route(world);

        let route_cat_to_dog_house = world.find_route(CAT_TO_DOG_HOUSE).unwrap();
        let backyard = world.find_area(testing::BACKYARD).unwrap(); // new Point B

        // edit route to change Point B from 'dog_house' to 'backyard'
        let mut world_editor = World::editor();
        let mut route_editor = route_cat_to_dog_house.edit_self();
        route_editor.point_b({
            let mut point_creator = Endpoint::creator();
            point_creator
                .end({
                    let mut end_creator = End::creator();
                    end_creator
                        .area_identity(IdentityBuilder::from_existing(&end_creator, backyard)).unwrap()
                        .direction(Direction::down()).unwrap()
                        .descriptor({
                            let mut descriptor_creator = Descriptor::creator();
                            descriptor_creator
                                .name(s!("portal_to_cat_house")).unwrap()
                                .description(s!("A portal swirling below")).unwrap();
                            descriptor_creator
                        }).unwrap();
                    end_creator
                }).unwrap();
            point_creator.point_builder()
        }).unwrap();

        world_editor.edit_route(route_editor).unwrap();
        world_editor.modify(&mut world).unwrap();

        let route_cat_to_dog_house = world.find_route(CAT_TO_DOG_HOUSE).unwrap();
        let cat_house = world.find_area(testing::CAT_HOUSE).unwrap(); // Point A
        let backyard = world.find_area(testing::BACKYARD).unwrap(); // new Point B
        let dog_house = world.find_area(testing::DOG_HOUSE).unwrap(); // old Point B

        assert!(cat_house.route_uids().contains(&route_cat_to_dog_house.uid()));
        assert!(backyard.route_uids().contains(&route_cat_to_dog_house.uid()));
        assert!(!dog_house.route_uids().contains(&route_cat_to_dog_house.uid()));

        if let Point::Endpoint(endpoint) = route_cat_to_dog_house.point_a() {
            assert_eq!(endpoint.end().area_uid(), cat_house.uid());
        } else {
            panic!("Route Point A should be an Endpoint");
        }

        if let Point::Endpoint(endpoint) = route_cat_to_dog_house.point_b() {
            assert_eq!(endpoint.end().area_uid(), backyard.uid());
        } else {
            panic!("Route Point B should be an Endpoint");
        }
    
    }

   #[test]
    pub fn test_remove_route() {
        let mut world = testing::create_world();
        world = create_route(world);

        let route_uid = world.find_route(CAT_TO_DOG_HOUSE).unwrap().uid();

        // remove the route 
        let mut world_editor = World::editor();
        world_editor.remove_route(route_uid).unwrap();
        world_editor.modify(&mut world).unwrap();

        let cat_house = world.find_area(testing::CAT_HOUSE).unwrap(); // Point A
        let dog_house = world.find_area(testing::DOG_HOUSE).unwrap(); // Point B

        assert!(matches!(world.find_route(CAT_TO_DOG_HOUSE), Err(_)));
        assert!(!cat_house.route_uids().contains(&route_uid));
        assert!(!dog_house.route_uids().contains(&route_uid));
    }
}

