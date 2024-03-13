use behavior::{Actor, Stimulation};
use model::{Builder, Built};
use tokio::{self, sync::mpsc};
use asmov_else_model as model;
use asmov_else_behavior as behavior;
use asmov_else_server_common as server;
use crate::universe_service::*;

pub struct UniverseRuntime {
    universe: model::Universe,
}

impl UniverseRuntime {
    pub fn load() -> model::Result<Self> {
        let (universe,_,_) = model::testing::create_universe();

        Ok(Self {
            universe,
        })
    }

    pub const fn universe(&self) -> &model::Universe {
        &self.universe
    }

    pub const fn tick_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(1)
    }

    pub async fn tick(&mut self) -> model::Result<()> {
        Ok(())
    }
}
