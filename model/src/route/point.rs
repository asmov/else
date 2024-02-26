use crate::{error::*, modeling::*, route::*, world::*};
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Point {
    Endpoint (Endpoint),
    Junction (Junction)
}

impl Point {
    pub fn area_uids(&self) -> Vec<UID> {
        match self {
            Point::Endpoint(endpoint) => vec![endpoint.end().area_uid()],
            Point::Junction(junction) => junction.entrances().iter().map(|end| end.area_uid()).collect()
        }
    }

    pub fn end_for_area(&self, area_uid: UID) -> Option<&End> {
        match self {
            Point::Endpoint(endpoint) => {
                if endpoint.end().area_uid() == area_uid {
                    Some(endpoint.end())
                } else {
                    None
                }
            },
            Point::Junction(junction) => {
                junction.entrances().iter()
                    .find(|end| end.area_uid() == area_uid)
            },
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum PointBuilder {
    Endpoint (EndpointBuilder),
    Junction (JunctionBuilder)
}

impl Builder for PointBuilder {
    type ModelType = Point;
    type BuilderType = Self;

    fn creator() -> Self {
        panic!("PointBuilder::creator() should not be called directly")
    }

    fn editor() -> Self {
        panic!("PointBuilder::creator() should not be called directly")
    }

    fn builder_mode(&self) -> BuilderMode {
        match self {
            PointBuilder::Endpoint(b) => b.builder_mode(),
            PointBuilder::Junction(b) => b.builder_mode(),
        }
    }

    fn create(self) -> Result<Creation<Self::BuilderType>> {
        match self {
            PointBuilder::Endpoint(b) => b.create(),
            PointBuilder::Junction(b) => b.create(),
        }
    }

    fn modify(self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        match self {
            PointBuilder::Endpoint(builder) => {
                if let Point::Endpoint(orig_endpoint) = existing {
                    builder.modify(orig_endpoint)
                } else {
                    unreachable!("Dispatch mismatch for PointBuilder::modify(Endpoint)");
                }
            },
            PointBuilder::Junction(builder) => {
                if let Point::Junction(orig_junction) = existing {
                    builder.modify(orig_junction)
                } else {
                    unreachable!("Dispatch mismatch for PointBuilder::modify(Junction)");
                }
            }
        }
    }

    fn class_ident(&self) -> &'static ClassIdent {
        match self {
            Self::Endpoint(modeler) => modeler.class_ident(),
            Self::Junction(modeler) => modeler.class_ident(),
        }
    }
}

impl CloneBuilding for PointBuilder {
    fn clone_model(builder_mode: BuilderMode, existing: &Self::ModelType) -> Self {
        match existing {
            Point::Endpoint(m) => PointBuilder::Endpoint(EndpointBuilder::clone_model(builder_mode, m)), 
            Point::Junction(m) => PointBuilder::Junction(JunctionBuilder::clone_model(builder_mode, m)),
        }
    }
}

impl PointBuilder {
    /// Retrieves all upcoming UIDs for the areas associated with this point
    /// Expects all UIDs to have been set by now.
    pub fn area_uids(&self) -> Result<Vec<UID>> {
        match self {
            PointBuilder::Endpoint(b) => {
                Ok(vec![
                    b.get_end_builder()
                        .expect("EndBuilder should exist")
                        .get_area_identity()
                        .expect("AreaIdentity should exist")
                        .try_uid()?
                ])
            },
            PointBuilder::Junction(b) => {
                b.entrances().iter()
                    .filter_map(|entrance_op| match entrance_op {
                        ListOp::Add(end) | ListOp::Edit(end) => {
                            Some(end.get_area_identity()
                                .expect("AreaIdentity should exist")
                                .try_uid())
                        },
                        ListOp::Remove(_) => None
                    })
                    .collect::<Result<Vec<_>>>()
            },
        }
    }
}

pub trait PointBuilderVariant: Builder {
    fn point_builder(self) -> PointBuilder;
}

