use crate::{character::*, interface::*};

pub struct WorldAction;
impl WorldAction {
    pub fn spawn_thing(world: &mut World, thing: ThingBuilder) -> Result<(UID, Modification<WorldBuilder>)> {
        let mut world_editor = world.edit_self();
        world_editor.add_thing(thing)?;
        let modification = world_editor.modify(world)?;
        let uid = modification.builder()
            .get_thing_ops()
            .last().unwrap()
            .added().unwrap()
            .try_uid().unwrap();

        Ok((uid, modification))
    }

    pub fn link_interface(world: &mut World, interface_uid: UID, character_uid: UID) -> Result<Modification<WorldBuilder>> {
        let interface = world.interface(interface_uid)?;
        let character = world.thing(character_uid)?.try_character()?;

        // perform validation
        if interface.downlinked() {
            return Err(Error::InterfaceAlreadyLinked{
                interface_uid,
                character_uid,
                linked_character_uid: interface.downlink_uid().unwrap()});
        } else if character.cortex().is_intelligent() {
            return Err(Error::CharacterAlreadyLinked {
                interface_uid,
                character_uid,
                linked_interface_uid: character.cortex().intelligence_lobe().unwrap().interface_uid() });
        } // todo: this is where reservations would be checked

        let mut interface_editor = interface.edit_self();
        interface_editor.downlink_uid(IdentityBuilder::editor_from_uid(character_uid))?;

        let mut character_editor = character.edit_self();
        character_editor.cortex_builder().set_intelligence_lobe({
            let mut intelligence_lobe = IntelligenceLobeBuilder::creator();
            intelligence_lobe.interface_uid(IdentityBuilder::editor_from_uid(interface_uid))?;
            intelligence_lobe
        })?;
        
        let mut world_editor = world.edit_self();
        world_editor.edit_interface(interface_editor)?;
        world_editor.edit_thing(character_editor.thing_builder())?;
        let modification = world_editor.modify(world)?;

        Ok(modification)
    }

}

#[cfg(test)]
mod tests {
    use crate::{interface::*, testing};
    use super::*;

    #[test]
    fn test_link_interface() {
        let mut world = testing::create_world();
        
        // create an interface
        let interface_builder = testing::interface_from_universe();
        let mut world_editor = world.edit_self();
        world_editor.add_interface(interface_builder).unwrap();
        world_editor.modify(&mut world).unwrap();

        // link the interface to HOUSEKEEPER
        let interface_uid = world.interfaces().last().unwrap().uid();
        let housekeeper_uid = world.find_thing(testing::HOUSEKEEPER).unwrap().uid();
        WorldAction::link_interface(&mut world, interface_uid, housekeeper_uid).unwrap();

        // check that the interface is linked to HOUSEKEEPER
        let interface = world.interface(interface_uid).unwrap();
        let housekeeper = world.thing(housekeeper_uid).unwrap().try_character().unwrap();
        assert_eq!(interface.downlink_uid().unwrap(), housekeeper_uid);
        assert_eq!(housekeeper.cortex().intelligence_lobe().unwrap().interface_uid(), interface_uid);
    }
}