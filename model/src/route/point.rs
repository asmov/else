use crate::{error::*, builder::*, route::*};
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Point {
    Endpoint (Endpoint),
    Junction (Junction)
}

#[derive(Debug)]
pub enum PointBuilder {
    Endpoint (EndpointBuilder),
    Junction (JunctionBuilder)
}

impl Builder for PointBuilder {
    type Type = Point;

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

    fn create(self) -> Result<Self::Type> {
        match self {
            PointBuilder::Endpoint(b) => Ok(Point::Endpoint(b.create()?)),
            PointBuilder::Junction(b) => Ok(Point::Junction(b.create()?)),
        }
    }

    fn modify(self, original: &mut Self::Type) -> Result<ModifyResult> {
        match self {
            PointBuilder::Endpoint(builder) => {
                if let Point::Endpoint(orig_endpoint) = original {
                    builder.modify(orig_endpoint)
                } else {
                    unreachable!("Dispatch mismatch for PointBuilder::modify(Endpoint)");
                }
            },
            PointBuilder::Junction(builder) => {
                if let Point::Junction(orig_junction) = original {
                    builder.modify(orig_junction)
                } else {
                    unreachable!("Dispatch mismatch for PointBuilder::modify(Junction)");
                }
            }
        }
    }
}

pub trait BuildablePoint: Builder {
    fn point_builder(self) -> PointBuilder;
}

