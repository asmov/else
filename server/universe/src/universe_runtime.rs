use behavior::{Actor, Stimulation};
use model::{Builder, Built};
use tokio::{self, sync::mpsc};
use asmov_else_model as model;
use asmov_else_behavior as behavior;
use asmov_else_server_common as server;
use crate::universe_service::*;

pub struct UniverseRuntime {
    universe: model::Universe,
    sync_channel_tx: Option<mpsc::Sender<model::Sync>>,
}

impl UniverseRuntime {
    pub fn load() -> model::Result<Self> {
        let (universe,_,_) = model::testing::create_universe();

        Ok(Self {
            universe,
            sync_channel_tx: None,
        })
    }

    pub fn subscribe_sync(&mut self) -> mpsc::Receiver<model::Sync> {
        let (tx, rx) = mpsc::channel(8);
        self.sync_channel_tx = Some(tx);
        rx
    }

    pub fn universe(&self) -> &model::Universe {
        &self.universe
    }

    pub async fn tick(&mut self) -> model::Result<()> {
        Ok(())
    }

    pub fn tick_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(1)
    }
}
