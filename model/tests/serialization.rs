#[cfg(test)]
mod tests {
    use elsezone_model::{self as model, Descriptive};
    use bincode;
    use std::fs::File;
    use std::io::{Write, Read, Seek, SeekFrom};
    use tempfile;

    #[test]
    fn test_world_binary() {
        let bincode_config = bincode::config::standard();
        let world_write = model::testing::create_world();
        let world_bytes_write = bincode::serde::encode_to_vec(&world_write, bincode_config).unwrap();
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write(&world_bytes_write).unwrap();
        tmpfile.seek(SeekFrom::Start(0)).unwrap();
        let mut world_bytes_read = Vec::new();
        tmpfile.read_to_end(&mut world_bytes_read).unwrap();
        let world_read: model::World = bincode::serde::decode_from_slice(&world_bytes_read, bincode_config).unwrap().0;

        assert_eq!(world_write.description(), world_read.description());
        assert_eq!(
            world_write.find_area("dog_house").unwrap().name(),
            world_read.find_area("dog_house").unwrap().name());
        assert_eq!(
            world_write.find_thing("black_cat").unwrap().name(),
            world_read.find_thing("black_cat").unwrap().name());
    }
}