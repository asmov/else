pub mod stimulus;

use elsezone_model::*;
use stimulus::*;


pub trait Stimulation {
    // Perceive a change in environment and (possibly) react to it.
    fn stimulate(&self, stimulus: Stimulus, world: &World) -> Result<Option<Vec<Reaction>>>;
}

pub trait Reactor {
    fn react(self, world: &mut World) -> Result<()>;
}

pub enum Reaction {
    Multiply(MultiplyReaction)
}

impl Reactor for Reaction {
    fn react(self, world: &mut World) -> Result<()> {
        match self {
            Reaction::Multiply(reaction) => reaction.react(world),
        }
    }
}

pub struct MultiplyReaction {
    frame: Frame,
    clone_id: ID
}

impl MultiplyReaction {
    fn new(frame: Frame, clone_id: ID) -> Self {
        Self {
            frame,
            clone_id,
        }
    }
}

impl Reactor for MultiplyReaction {
    fn react(self, world: &mut World) -> Result<()> {
        let thing = world.thing(self.clone_id).unwrap();
        let mut character = CharacterBuilder::creator(); 
        character.entity({
            let mut entity = EntityBuilder::creator();
            entity.descriptor({
                let mut descriptor = DescriptorBuilder::creator();
                descriptor.key(format!("{}_{}", thing.name(), self.frame))?;
                descriptor
            })?;
            entity
        })?;

        let _id = world.spawn_thing(character.thing_builder(), 1).unwrap();
        Ok(())
    }
}

pub struct VoidCharacterRoutine(Identity);
impl Stimulation for VoidCharacterRoutine {
    fn stimulate(&self, _stimulus: Stimulus, _world: &World) -> Result<Option<Vec<Reaction>>> { Ok(None) }
}

pub struct MultiplierCharacterRoutine(Identity);
impl Stimulation for MultiplierCharacterRoutine {
    fn stimulate(&self, stimulus: Stimulus, _world: &World) -> Result<Option<Vec<Reaction>>> {
        match stimulus {
            Stimulus::Time(timeframe) => {
                Ok(Some(vec![Reaction::Multiply(MultiplyReaction::new(timeframe.frame(), self.0.id()))]))
            },
            _ => { Ok(None) }
        }
    }
}


pub enum CharacterRoutine {
    Void(VoidCharacterRoutine),
    Multiplier(MultiplierCharacterRoutine)
}

impl Stimulation for CharacterRoutine {
    fn stimulate(&self, stimulus: Stimulus, world: &World) -> Result<Option<Vec<Reaction>>> {
        match self {
            CharacterRoutine::Void(routine) => routine.stimulate(stimulus, world),
            CharacterRoutine::Multiplier(routine) => routine.stimulate(stimulus, world),
        }
    }
}

impl CharacterRoutine {
    const ID_VOID: RoutineID = 0;
    const ID_MULTIPLIER: RoutineID = 1;

    pub fn new(character: &Character) -> CharacterRoutine {
        let routine_id = character.routine_id();
        let identity = character.identity().clone();

        match routine_id {
            Self::ID_VOID => Self::Void(VoidCharacterRoutine(identity)),
            Self::ID_MULTIPLIER => Self::Multiplier(MultiplierCharacterRoutine(identity)),
            _ => panic!("Unknown routine ID: {routine_id}")
        }
    }
}

pub struct WorldRuntime {
    timeframe: TimeFrame,
    world: World,
    character_routines: Vec<CharacterRoutine>,
}

impl WorldRuntime {
    pub fn load() -> Result<Self> {
        let world = testing::create_world();
        let character_routines = world.things().iter()
            .filter_map(|thing| match thing {
                Thing::Character(c) => Some(c),
                _ => None
            })
            .map(|character| CharacterRoutine::new(character))
            .collect();

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap()
            .as_secs();

        Ok(Self {
            timeframe: TimeFrame::new(0, now),
            world,
            character_routines
        })
    }

    pub fn timeframe(&self) -> &TimeFrame {
        &self.timeframe
    }

    pub fn tick(&mut self) -> Result<()> {
        self.timeframe.tick();
        let reactions = self.on_timeframe(self.timeframe.clone())?;
        self.react(reactions)?;
        Ok(())
    }

    fn on_timeframe(&mut self, timeframe: TimeFrame) -> Result<Vec<Reaction>> {
        let mut reactions = Vec::new();

        for routine in &mut self.character_routines {
            let world = &mut self.world;
            let result = routine.stimulate(Stimulus::Time(&timeframe), world)?;
            if let Some(mut results) = result {
                reactions.append(&mut results);
            }
        }

        Ok(reactions)
    }

    fn react(&mut self, reactions: Vec<Reaction>) -> Result<()> {
        for reaction in reactions {
            reaction.react(&mut self.world)?
        }

        Ok(())
    }
}