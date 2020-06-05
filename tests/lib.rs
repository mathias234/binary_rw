extern crate binary_rw;

use binary_rw::{BinaryReader, BinaryWriter, OpenType};

#[test]
fn seek_test() {
    {
        let mut writer = BinaryWriter::new("seek.dat", OpenType::OpenAndCreate)
            .expect("Failed to create writer");

        let temp: f32 = 50.0;
        let seek_loc = 5;

        writer.write_bytes([16; 32].to_vec());
        writer.seek_to(seek_loc).expect("Writer seek error");
        assert_eq!(writer.get_cur_pos().expect("Failed to get pos"), seek_loc);
        writer.write_f32(temp);

        let mut reader =
            BinaryReader::new("seek.dat", OpenType::Open).expect("Failed to open reader");

        reader.seek_to(seek_loc).expect("Writer seek error");
        assert_eq!(reader.get_cur_pos().expect("Failed to get pos"), seek_loc);
        let read_temp = reader.read_f32().expect("Failed to read file");

        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./seek.dat").unwrap();
}

#[test]
fn read_write_test_f64() {
    {
        let mut writer =
            BinaryWriter::new("f64.dat", OpenType::OpenAndCreate).expect("Failed to create writer");

        let temp: f64 = 50.0;

        writer.write_f64(temp);

        let mut reader =
            BinaryReader::new("f64.dat", OpenType::Open).expect("Failed to open reader");

        let read_temp = reader.read_f64().expect("Failed to read file");

        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./f64.dat").unwrap();
}

#[test]
fn read_write_test_f32() {
    {
        let mut writer =
            BinaryWriter::new("f32.dat", OpenType::OpenAndCreate).expect("Failed to create writer");

        let temp: f32 = 50.0;

        writer.write_f32(temp);

        let mut reader =
            BinaryReader::new("f32.dat", OpenType::Open).expect("Failed to open reader");

        let read_temp = reader.read_f32().expect("Failed to read file");

        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./f32.dat").unwrap();
}

#[test]
fn read_write_test_isize() {
    {
        let mut writer = BinaryWriter::new("isize.dat", OpenType::OpenAndCreate)
            .expect("Failed to create writer");

        let temp: isize = 50;

        writer.write_isize(temp);

        let mut reader =
            BinaryReader::new("isize.dat", OpenType::Open).expect("Failed to open reader");

        let read_temp = reader.read_isize().expect("Failed to read file");

        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./isize.dat").unwrap();
}

#[test]
fn read_write_test_usize() {
    {
        let mut writer = BinaryWriter::new("usize.dat", OpenType::OpenAndCreate)
            .expect("Failed to create writer");

        let temp: usize = 50;

        writer.write_usize(temp);

        let mut reader =
            BinaryReader::new("usize.dat", OpenType::Open).expect("Failed to open reader");

        let read_temp = reader.read_usize().expect("Failed to read file");
        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./usize.dat").unwrap();
}

#[test]
fn read_write_test_i64() {
    {
        let mut writer =
            BinaryWriter::new("i64.dat", OpenType::OpenAndCreate).expect("Failed to create writer");

        let temp: i64 = 50;

        writer.write_i64(temp);

        let mut reader =
            BinaryReader::new("i64.dat", OpenType::Open).expect("Failed to open reader");

        let read_temp = reader.read_i64().expect("Failed to read file");

        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./i64.dat").unwrap();
}

#[test]
fn read_write_test_i32() {
    {
        let mut writer =
            BinaryWriter::new("i32.dat", OpenType::OpenAndCreate).expect("Failed to create writer");

        let temp: i32 = 50;

        writer.write_i32(temp);

        let mut reader =
            BinaryReader::new("i32.dat", OpenType::Open).expect("Failed to open reader");

        let read_temp = reader.read_i32().expect("Failed to read file");

        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./i32.dat").unwrap();
}

#[test]
fn read_write_test_i16() {
    {
        let mut writer =
            BinaryWriter::new("i16.dat", OpenType::OpenAndCreate).expect("Failed to create writer");

        let temp: i16 = 50;

        writer.write_i16(temp);

        let mut reader =
            BinaryReader::new("i16.dat", OpenType::Open).expect("Failed to open reader");

        let read_temp = reader.read_i16().expect("Failed to read file");

        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./i16.dat").unwrap();
}

#[test]
fn read_write_test_i8() {
    {
        let mut writer =
            BinaryWriter::new("i8.dat", OpenType::OpenAndCreate).expect("Failed to create writer");

        let temp: i8 = 50;

        writer.write_i8(temp);

        let mut reader =
            BinaryReader::new("i8.dat", OpenType::Open).expect("Failed to open reader");

        let read_temp = reader.read_i8().expect("Failed to read file");

        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./i8.dat").unwrap();
}

#[test]
fn read_write_test_u64() {
    {
        let mut writer =
            BinaryWriter::new("u64.dat", OpenType::OpenAndCreate).expect("Failed to create writer");

        let temp: u64 = 50;

        writer.write_u64(temp);

        let mut reader =
            BinaryReader::new("u64.dat", OpenType::Open).expect("Failed to open reader");

        let read_temp = reader.read_u64().expect("Failed to read file");

        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./u64.dat").unwrap();
}

#[test]
fn read_write_test_u32() {
    {
        let mut writer =
            BinaryWriter::new("u32.dat", OpenType::OpenAndCreate).expect("Failed to create writer");

        let temp: u32 = 50;

        writer.write_u32(temp);

        let mut reader =
            BinaryReader::new("u32.dat", OpenType::Open).expect("Failed to open reader");

        let read_temp = reader.read_u32().expect("Failed to read file");

        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./u32.dat").unwrap();
}

#[test]
fn read_write_test_u16() {
    {
        let mut writer =
            BinaryWriter::new("u16.dat", OpenType::OpenAndCreate).expect("Failed to create writer");

        let temp: u16 = 50;

        writer.write_u16(temp);

        let mut reader =
            BinaryReader::new("u16.dat", OpenType::Open).expect("Failed to open reader");

        let read_temp = reader.read_u16().expect("Failed to read file");

        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./u16.dat").unwrap();
}

#[test]
fn read_write_test_u8() {
    {
        let mut writer =
            BinaryWriter::new("u8.dat", OpenType::OpenAndCreate).expect("Failed to create writer");

        let temp: u8 = 50;

        writer.write_u8(temp);

        let mut reader =
            BinaryReader::new("u8.dat", OpenType::Open).expect("Failed to open reader");

        let read_temp = reader.read_u8().expect("Failed to read file");

        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./u8.dat").unwrap();
}

#[test]
fn read_write_bytes() {
    {
        let mut writer = BinaryWriter::new("bytes.dat", OpenType::OpenAndCreate)
            .expect("Failed to create writer");

        let count = 20;

        let temp = vec![16; count];

        writer.write_bytes(temp.clone());

        let mut reader =
            BinaryReader::new("bytes.dat", OpenType::Open).expect("Failed to open reader");

        let read_temp = reader
            .read_bytes(count as u64)
            .expect("Failed to read file");

        assert_eq!(temp, read_temp);
    }

    std::fs::remove_file("./bytes.dat").unwrap();
}
