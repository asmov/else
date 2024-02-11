use behavior::{Reactor, Stimulation};
use tokio;
use elsezone_model as model;
use elsezone_behavior as behavior;

pub struct WorldRuntime {
    timeframe: model::TimeFrame,
    world: model::World,
    character_routines: Vec<behavior::CharacterRoutine>,
    timeframe_channel_tx: tokio::sync::watch::Sender<model::TimeFrame> 
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
            timeframe_channel_tx: tokio::sync::watch::channel(model::TimeFrame::new(0,0)).0
        })
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

    pub fn tick(&mut self) -> model::Result<()> {
        self.timeframe.tick();
        let reactions = self.on_timeframe(self.timeframe.clone())?;
        self.react(reactions)?;
        self.timeframe_channel_tx.send(self.timeframe.clone()).unwrap();
        Ok(())
    }

    fn on_timeframe(&mut self, timeframe: model::TimeFrame) -> model::Result<Vec<behavior::Reaction>> {
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

    fn react(&mut self, reactions: Vec<behavior::Reaction>) -> model::Result<()> {
        for reaction in reactions {
            reaction.react(&mut self.world)?
        }

        Ok(())
    }
}