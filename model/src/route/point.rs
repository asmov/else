use crate::{error::*, builder::*, route::*};
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Point {
    Endpoint (Endpoint),
    Junction (Junction)
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

    fn modify(self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
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

