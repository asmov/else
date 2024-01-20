#[cfg(test)]
mod tests {
    use elsezone_model::{self as model, Descriptive};
    use bincode;
    use std::fs::File;
    use std::io::{Write, Read, Seek, SeekFrom};
    use tempfile;

    #[test]
    fn test_zone_binary() {
        let zone_write = model::testing::create_zone();
        let zone_bytes_write = bincode::serialize(&zone_write).unwrap();
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write(&zone_bytes_write).unwrap();
        tmpfile.seek(SeekFrom::Start(0)).unwrap();
        let mut zone_bytes_read = Vec::new();
        tmpfile.read_to_end(&mut zone_bytes_read).unwrap();
        let zone_read: model::Zone = bincode::deserialize(&zone_bytes_read).unwrap();

        assert_eq!(zone_write.description(), zone_read.description());
        assert_eq!(
            zone_write.find_area("dog_house").unwrap().name(),
            zone_read.find_area("dog_house").unwrap().name());
        assert_eq!(
            zone_write.find_thing("black_cat").unwrap().name(),
            zone_read.find_thing("black_cat").unwrap().name());
    }
}