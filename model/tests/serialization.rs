#[cfg(test)]
mod tests {
    use elsezone_model::{self as model, Descriptive};
    use bincode;
    use model::{testing, BuildableAreaVector, BuildableDescriptor, Builder, Built, DomainSynchronizer, Identifiable};
    use std::fs::File;
    use std::io::{Write, Read, Seek, SeekFrom};
    use tempfile;

    fn setup_world_io() -> (model::World, File, model::World) {
        let bincode_config = bincode::config::standard();
        let world_write = model::testing::create_world();
        let world_bytes_write = bincode::serde::encode_to_vec(&world_write, bincode_config).unwrap();
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write(&world_bytes_write).unwrap();
        tmpfile.seek(SeekFrom::Start(0)).unwrap();
        let mut world_bytes_read = Vec::new();
        tmpfile.read_to_end(&mut world_bytes_read).unwrap();
        tmpfile.seek(SeekFrom::Start(0)).unwrap();
        let world_read: model::World = bincode::serde::decode_from_slice(&world_bytes_read, bincode_config).unwrap().0;
        (world_write, tmpfile, world_read)
    }

    #[test]
    fn test_world_binary() {
        let (world_write, _tempfile, world_read) = setup_world_io();

        assert_eq!(world_write.description(), world_read.description());
        assert_eq!(
            world_write.find_area(testing::DOG_HOUSE).unwrap().name(),
            world_read.find_area(testing::DOG_HOUSE).unwrap().name());
        assert_eq!(
            world_write.find_thing("black_cat").unwrap().name(),
            world_read.find_thing("black_cat").unwrap().name());
    }

    #[test]
    fn test_modification_binary() {
        const NEW_DESCRIPTION: &'static str = "We have changed the description";
        let binconfig = bincode::config::standard();

        let (mut world_upstream, _tempfile, mut world_downstream) = setup_world_io();

        // change the area description on the upstream world
        let mut world_editor = world_upstream.edit_self();
        let mut area_editor = world_upstream
            .find_area(testing::DOG_HOUSE).unwrap()
            .edit_self();
        area_editor.descriptor_builder().description(NEW_DESCRIPTION.to_string()).unwrap();
        world_editor.edit_area(area_editor).unwrap();
        let modification = world_editor.modify(&mut world_upstream).unwrap();

        // verify that the area description has changed on the upstream world
        let area_dog_house = world_upstream.find_area(model::testing::DOG_HOUSE).unwrap();
        assert_eq!(NEW_DESCRIPTION, area_dog_house.description().unwrap());

        // "send" the modification downstream 
        let upstream_sync = model::Sync::World(model::Operation::Modification(modification));
        let sync_bytes = bincode::serde::encode_to_vec(upstream_sync, binconfig).unwrap();
        let downstream_sync: model::Sync  = bincode::serde::decode_from_slice(&sync_bytes, binconfig).unwrap().0;

        // update the downstream
        downstream_sync.sync(&mut world_downstream).unwrap();

        // verify that downstream has been updated
        let area_dog_house = world_downstream.find_area(testing::DOG_HOUSE).unwrap();
        assert_eq!(NEW_DESCRIPTION, area_dog_house.description().unwrap());

        // verify that both worlds are now equal bitwise
        let upstream_bytes = bincode::serde::encode_to_vec(world_upstream, binconfig).unwrap();
        let downstream_bytes = bincode::serde::encode_to_vec(world_downstream, binconfig).unwrap();
        assert_eq!(upstream_bytes, downstream_bytes);
    }
}