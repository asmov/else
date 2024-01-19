#[cfg(test)]
mod tests {
    use elserpg_model::{self as model, Descriptive};
    use bincode;
    use std::fs::File;
    use std::io::{Write, Read, Seek, SeekFrom};
    use tempfile;

    #[test]
    fn test_world_binary() {
        let world_write = model::testing::create_world();
        let world_bytes_write = bincode::serialize(&world_write).unwrap();
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write(&world_bytes_write).unwrap();
        tmpfile.seek(SeekFrom::Start(0)).unwrap();
        let mut world_bytes_read = Vec::new();
        tmpfile.read_to_end(&mut world_bytes_read).unwrap();
        let world_read: model::World = bincode::deserialize(&world_bytes_read).unwrap();

        assert_eq!(world_write.description(), world_read.description());
        assert_eq!(
            world_write.find_area("dog_house").unwrap().name(),
            world_read.find_area("dog_house").unwrap().name());
        assert_eq!(
            world_write.find_thing("black_cat").unwrap().name(),
            world_read.find_thing("black_cat").unwrap().name());
    }
}