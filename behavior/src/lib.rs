pub mod stimulus;

use elsezone_model::*;
use stimulus::*;


pub trait Stimulation {
    // Perceive a change in environment and (possibly) react to it.
    fn stimulate(&mut self, stimulus: Stimulus, world: &mut World);
}

pub struct VoidCharacterRoutine(Identity);
impl Stimulation for VoidCharacterRoutine {
    fn stimulate(&mut self, stimulus: Stimulus, world: &mut World) {}
}

pub struct MultiplierCharacterRoutine(Identity);
impl Stimulation for MultiplierCharacterRoutine {
    fn stimulate(&mut self, stimulus: Stimulus, world: &mut World) {
        match stimulus {
            Stimulus::Time(timeframe) => {
                let myself = world.thing(self.0.id()).unwrap();
                let mut character = CharacterBuilder::creator(); 
                character.entity({
                    let mut entity = EntityBuilder::creator();
                    entity.descriptor({
                        let mut descriptor = DescriptorBuilder::creator();
                        descriptor.key(format!("{}_{}", myself.name(), timeframe.frame()));
                        descriptor
                    });
                    entity
                });

                let _id = world.spawn_thing(character.thing_builder(), 1).unwrap();
            },
            _ => {}
        }
    }
}


pub enum CharacterRoutine {
    Void(VoidCharacterRoutine),
    Multiplier(MultiplierCharacterRoutine)
}

impl Stimulation for CharacterRoutine {
    fn stimulate(&mut self, stimulus: Stimulus, world: &mut World) {
        match self {
            CharacterRoutine::Void(routine) => routine.stimulate(stimulus, world),
            CharacterRoutine::Multiplier(routine) => routine.stimulate(stimulus, world),
        }
    }
}

impl CharacterRoutine {
    fn new(character: &Character) -> CharacterRoutine {
        let routine_id = character.routine_id();
        let identity = character.identity().clone();
        match routine_id {
            0 => Self::Void(VoidCharacterRoutine(identity)),
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
    pub fn load() -> Self {
        let world = testing::create_world();
        let character_routines = world.things().iter()
            .filter_map(|thing| match thing {
                Thing::Character(c) => Some(c),
                _ => None
            })
            .map(|character| CharacterRoutine::new(character))
            .collect();

        Self {
            timeframe: TimeFrame::new(0, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()),
            world,
            character_routines
        }
    }

    pub fn on_timeframe(&mut self, timeframe: TimeFrame) {
        for routine in &mut self.character_routines {
            let world = &mut self.world;
            routine.stimulate(Stimulus::Time(&timeframe), world);
        }
    }
}
