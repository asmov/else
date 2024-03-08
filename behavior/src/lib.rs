pub mod stimulus;

use asmov_else_model::*;

pub use stimulus::*;


pub trait Stimulation {
    // Perceive a change in environment and (possibly) react to it.
    fn stimulate(&self, stimulus: Stimulus, world: &World) -> Result<Option<Vec<Action>>>;
}

pub trait Actor {
    fn act(self, world: &mut World) -> Result<Vec<Sync>>;
}

impl Actor for Action {
    fn act(self, world: &mut World) -> Result<Vec<Sync>> {
        match self {
            Action::Multiply(action) => action.act(world),
            Action::Go(action) => action.act(world),
        }
    }
}

impl Actor for GoAction {
    fn act(self, world: &mut World) -> Result<Vec<Sync>> {
        todo!()
    }
}

impl Actor for MultiplyAction {
    fn act(self, world: &mut World) -> Result<Vec<Sync>> {
        let thing = world.thing(self.base.thing_uid).unwrap();
        let mut character = CharacterBuilder::creator(); 
        character.entity({
            let mut entity = EntityBuilder::creator();
            entity.descriptor({
                let mut descriptor = DescriptorBuilder::creator();
                descriptor.key(format!("{}_{}", thing.name(), self.base.frame))?;
                descriptor
            })?;
            entity.location(thing.location()).unwrap();
            entity
        })?;

        let _ = WorldAction::spawn_thing(world, character.thing_builder()).unwrap();
        todo!()
        //Ok(Vec::new())
    }
}

pub struct VoidCharacterRoutine(UID);
impl Stimulation for VoidCharacterRoutine {
    fn stimulate(&self, _stimulus: Stimulus, world: &World) -> Result<Option<Vec<Action>>> {
        let thing = world.thing(self.0).unwrap();
        println!("< I am {} and I have nothing to do. >", thing.descriptor().name());
        Ok(None)
    }
}

pub struct MultiplierCharacterRoutine(UID);
impl Stimulation for MultiplierCharacterRoutine {
    fn stimulate(&self, stimulus: Stimulus, _world: &World) -> Result<Option<Vec<Action>>> {
        match stimulus {
            Stimulus::Time(timeframe) => {
                Ok(Some(vec![Action::Multiply(MultiplyAction { base: BaseAction { frame: timeframe.frame(), thing_uid: self.0} })]))
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
    fn stimulate(&self, stimulus: Stimulus, world: &World) -> Result<Option<Vec<Action>>> {
        match self {
            CharacterRoutine::Void(routine) => routine.stimulate(stimulus, world),
            CharacterRoutine::Multiplier(routine) => routine.stimulate(stimulus, world),
        }
    }
}

impl CharacterRoutine {
    const ID_VOID: UID = 0;
    const ID_MULTIPLIER: UID = 1;

    pub fn new(character: &Character) -> CharacterRoutine {
        let routine_id = character.cortex().routine_uid();
        let uid = character.uid();

        match routine_id {
            Self::ID_VOID => Self::Void(VoidCharacterRoutine(uid)),
            Self::ID_MULTIPLIER => Self::Multiplier(MultiplierCharacterRoutine(uid)),
            _ => panic!("Unknown routine ID: {routine_id}")
        }
    }
}

