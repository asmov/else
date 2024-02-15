pub mod stimulus;

use elsezone_model::*;

pub use stimulus::*;


pub trait Stimulation {
    // Perceive a change in environment and (possibly) react to it.
    fn stimulate(&self, stimulus: Stimulus, world: &World) -> Result<Option<Vec<Action>>>;
}

pub trait Actor {
    fn act(self, world: &mut World) -> Result<Vec<Sync>>;
}

pub enum Action {
    Multiply(MultiplyAction),
}

impl Actor for Action {
    fn act(self, world: &mut World) -> Result<Vec<Sync>> {
        match self {
            Action::Multiply(action) => action.act(world),
        }
    }
}

pub struct GoAction {
    thing_id: ID,
    frame_started: Frame,
    from_area_id: ID,
    thru_route_id: ID
}

impl Actor for GoAction {
    fn act(self, world: &mut World) -> Result<Vec<Sync>> {
        todo!()
    }
}

impl GoAction {
    pub fn new(thing_id: ID, frame_started: Frame, from_area_id: ID, thru_route_id: ID) -> Self {
        Self {
            thing_id,
            frame_started,
            from_area_id,
            thru_route_id,
        }
    }
}



pub struct MultiplyAction {
    frame: Frame,
    clone_id: ID
}

impl MultiplyAction {
    fn new(frame: Frame, clone_id: ID) -> Self {
        Self {
            frame,
            clone_id,
        }
    }
}

impl Actor for MultiplyAction {
    fn act(self, world: &mut World) -> Result<Vec<Sync>> {
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
        todo!()
        //Ok(Vec::new())
    }
}

pub struct VoidCharacterRoutine(Identity);
impl Stimulation for VoidCharacterRoutine {
    fn stimulate(&self, _stimulus: Stimulus, world: &World) -> Result<Option<Vec<Action>>> {
        let thing = world.thing(self.0.id()).unwrap();
        println!("< I am {} and I have nothing to do. >", thing.descriptor().name());
        Ok(None)
    }
}

pub struct MultiplierCharacterRoutine(Identity);
impl Stimulation for MultiplierCharacterRoutine {
    fn stimulate(&self, stimulus: Stimulus, _world: &World) -> Result<Option<Vec<Action>>> {
        match stimulus {
            Stimulus::Time(timeframe) => {
                Ok(Some(vec![Action::Multiply(MultiplyAction::new(timeframe.frame(), self.0.id()))]))
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

