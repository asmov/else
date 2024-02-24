use super::*;
use crate::{error::*, modeling::*, sync::*};

impl SynchronizedDomainBuilder<World> for WorldBuilder {
    fn synchronize(self, world: &mut World) -> Result<Modification<Self::BuilderType>> {
        self.modify(world)
    }
}

impl SynchronizedDomainBuilder<World> for AreaBuilder {
    fn synchronize(self, world: &mut World) -> Result<Modification<Self::BuilderType>> {
        let area_uid = self.try_uid()?;
        let area_mut = world.area_mut(area_uid)?;
        self.modify(area_mut)
    }
}

