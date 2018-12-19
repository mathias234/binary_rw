extern crate binary_rw;

use binary_rw::{BinaryReader, BinaryWriter, OpenType};

#[test]
fn read_write_test() {
    {
        let mut writer = BinaryWriter::new("test.dat", OpenType::OpenAndCreate);

        let temp: f32 = 50.0;

        writer.write_f32(temp);

        let mut reader = BinaryReader::new("test.dat", OpenType::Open);

        let read_temp = reader.read_f32();

        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./test.dat").unwrap();
}
