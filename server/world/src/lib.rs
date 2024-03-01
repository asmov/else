use behavior::{Actor, Stimulation};
use model::{Builder, Built};
use tokio;
use asmov_else_model as model;
use asmov_else_behavior as behavior;
use asmov_else_server_common as server;

pub struct WorldRuntime {
    timeframe: model::TimeFrame,
    world: model::World,
    character_routines: Vec<behavior::CharacterRoutine>,
    timeframe_channel_tx: tokio::sync::watch::Sender<model::TimeFrame>,
    sync_channel_tx: Option<tokio::sync::mpsc::Sender<model::Sync>>,
}

impl WorldRuntime {
    pub fn load() -> model::Result<Self> {
        let world = model::testing::create_world();
        let character_routines = world.things().iter()
            .filter_map(|thing| match thing {
                model::Thing::Character(c) => Some(c),
                _ => None
            })
            .map(|character| behavior::CharacterRoutine::new(character))
            .collect();

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap()
            .as_secs();


        Ok(Self {
            timeframe: model::TimeFrame::new(0, now),
            world,
            character_routines,
            timeframe_channel_tx: tokio::sync::watch::channel(model::TimeFrame::new(0,0)).0,
            sync_channel_tx: None,
        })
    }

    pub fn subscribe_sync(&mut self) -> tokio::sync::mpsc::Receiver<model::Sync> {
        let (tx, rx) = tokio::sync::mpsc::channel(8);
        self.sync_channel_tx = Some(tx);
        rx
    }

    pub fn subscribe_timeframe(&mut self) -> tokio::sync::watch::Receiver<model::TimeFrame> {
        self.timeframe_channel_tx.subscribe()
    }

    pub fn world(&self) -> &model::World {
        &self.world
    }

    pub fn timeframe(&self) -> &model::TimeFrame {
        &self.timeframe
    }

    pub fn frame_duration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(10)
    }

    pub async fn tick(&mut self) -> model::Result<()> {
        self.timeframe.tick();

        {
            let mut world_editor = model::World::editor();
            world_editor.frame(self.timeframe.frame())?;
            let modification = world_editor.modify(&mut self.world)?;
            if let Some(sync_tx) = &self.sync_channel_tx {
                let sync = model::Sync::World(model::Operation::Modification(modification));
                let _ = sync_tx.send(sync).await;
            }
        }

        let reactions = self.on_timeframe(self.timeframe.clone())?;
        self.react(reactions)?;
        let _ = self.timeframe_channel_tx.send(self.timeframe.clone());
        Ok(())
    }

    fn on_timeframe(&mut self, timeframe: model::TimeFrame) -> model::Result<Vec<model::Action>> {
        let mut reactions = Vec::new();

        for routine in &mut self.character_routines {
            let world = &mut self.world;
            let result = routine.stimulate(behavior::Stimulus::Time(&timeframe), world)?;
            if let Some(mut results) = result {
                reactions.append(&mut results);
            }
        }

        Ok(reactions)
    }

    fn react(&mut self, reactions: Vec<model::Action>) -> model::Result<Vec<model::Sync>> {
        let mut syncs = Vec::new();

        for reaction in reactions {
            syncs.append(&mut reaction.act(&mut self.world)?);
        }

        Ok(syncs)
    }
}