use binary_rw::{BinaryReader, BinaryWriter, Filestream, Memorystream, OpenType};

fn create_writer_stream(name: &str) -> Filestream {
    let name = format!("{}.test", name);
    Filestream::new(&name, OpenType::OpenAndCreate).expect("Failed to open stream")
}

fn create_reader_stream(name: &str) -> Filestream {
    let name = format!("{}.test", name);
    Filestream::new(&name, OpenType::Open).expect("Failed to open stream")
}

fn cleanup(name: &str) {
    let name = format!("{}.test", name);
    std::fs::remove_file(&name).expect("Failure to delete file");
}

#[test]
fn seek_test() {
    let temp: f32 = 50.0;
    let seek_loc = 5;

    let mut stream = create_writer_stream("seek");
    let mut writer = BinaryWriter::new(&mut stream);

    writer
        .write_bytes([16; 32].to_vec())
        .expect("Failed to write bytes");
    writer.seek_to(seek_loc).expect("Writer seek error");
    assert_eq!(writer.get_cur_pos().expect("Failed to get pos"), seek_loc);
    writer.write_f32(temp).expect("Failed to write f32");

    let mut stream = create_reader_stream("seek");
    let mut reader = BinaryReader::new(&mut stream);
    reader.seek_to(seek_loc).expect("Writer seek error");
    assert_eq!(reader.get_cur_pos().expect("Failed to get pos"), seek_loc);
    let read_temp = reader.read_f32().expect("Failed to read file");

    assert_eq!(temp, read_temp);

    cleanup("seek");
}

#[test]
fn read_write_test_f64() {
    let temp: f64 = 50.0;
    let mut stream = create_writer_stream("f64");
    let mut writer = BinaryWriter::new(&mut stream);

    writer.write_f64(temp).expect("Failed to write f64");

    let mut stream = create_reader_stream("f64");
    let mut reader = BinaryReader::new(&mut stream);

    let read_temp = reader.read_f64().expect("Failed to read file");

    assert_eq!(temp, read_temp);

    cleanup("f64")
}

#[test]
fn read_write_test_f32() {
    let temp: f32 = 50.0;
    let mut stream = create_writer_stream("f32");
    let mut writer = BinaryWriter::new(&mut stream);

    writer.write_f32(temp).expect("Failed to write f32");

    let mut stream = create_reader_stream("f32");
    let mut reader = BinaryReader::new(&mut stream);

    let read_temp = reader.read_f32().expect("Failed to read file");

    assert_eq!(temp, read_temp);

    cleanup("f32");
}

#[test]
fn read_write_test_isize() {
    let temp: isize = 50;
    let mut stream = create_writer_stream("isize");
    let mut writer = BinaryWriter::new(&mut stream);

    writer.write_isize(temp).expect("Failed to write isize");

    let mut stream = create_reader_stream("isize");
    let mut reader = BinaryReader::new(&mut stream);

    let read_temp = reader.read_isize().expect("Failed to read file");

    assert_eq!(temp, read_temp);

    cleanup("isize");
}

#[test]
fn read_write_test_usize() {
    let temp: usize = 50;
    let mut stream = create_writer_stream("usize");
    let mut writer = BinaryWriter::new(&mut stream);

    writer.write_usize(temp).expect("Failed to write usize");

    let mut stream = create_reader_stream("usize");
    let mut reader = BinaryReader::new(&mut stream);

    let read_temp = reader.read_usize().expect("Failed to read file");
    assert_eq!(temp, read_temp);

    cleanup("usize");
}

#[test]
fn read_write_test_i64() {
    let temp: i64 = 50;
    let mut stream = create_writer_stream("i64");
    let mut writer = BinaryWriter::new(&mut stream);

    writer.write_i64(temp).expect("Failed to write i64");

    let mut stream = create_reader_stream("i64");
    let mut reader = BinaryReader::new(&mut stream);

    let read_temp = reader.read_i64().expect("Failed to read file");

    assert_eq!(temp, read_temp);

    cleanup("i64");
}

#[test]
fn read_write_test_i32() {
    let temp: i32 = 50;
    let mut stream = create_writer_stream("i32");
    let mut writer = BinaryWriter::new(&mut stream);

    writer.write_i32(temp).expect("Failed to write i32");

    let mut stream = create_reader_stream("i32");
    let mut reader = BinaryReader::new(&mut stream);

    let read_temp = reader.read_i32().expect("Failed to read file");

    assert_eq!(temp, read_temp);

    cleanup("i32");
}

#[test]
fn read_write_test_i16() {
    let temp: i16 = 50;
    let mut stream = create_writer_stream("i16");
    let mut writer = BinaryWriter::new(&mut stream);

    writer.write_i16(temp).expect("Failed to write i16");

    let mut stream = create_reader_stream("i16");
    let mut reader = BinaryReader::new(&mut stream);

    let read_temp = reader.read_i16().expect("Failed to read file");

    assert_eq!(temp, read_temp);

    cleanup("i16");
}

#[test]
fn read_write_test_i8() {
    let temp: i8 = 50;
    let mut stream = create_writer_stream("i8");
    let mut writer = BinaryWriter::new(&mut stream);

    writer.write_i8(temp).expect("Failed to write i8");

    let mut stream = create_reader_stream("i8");
    let mut reader = BinaryReader::new(&mut stream);

    let read_temp = reader.read_i8().expect("Failed to read file");

    assert_eq!(temp, read_temp);

    cleanup("i8");
}

#[test]
fn read_write_test_u64() {
    let temp: u64 = 50;
    let mut stream = create_writer_stream("u64");
    let mut writer = BinaryWriter::new(&mut stream);

    writer.write_u64(temp).expect("Failed to write u64");

    let mut stream = create_reader_stream("u64");
    let mut reader = BinaryReader::new(&mut stream);

    let read_temp = reader.read_u64().expect("Failed to read file");

    assert_eq!(temp, read_temp);

    cleanup("u64");
}

#[test]
fn read_write_test_u32() {
    let temp: u32 = 50;
    let mut stream = create_writer_stream("u32");
    let mut writer = BinaryWriter::new(&mut stream);

    writer.write_u32(temp).expect("Failed to write u32");

    let mut stream = create_reader_stream("u32");
    let mut reader = BinaryReader::new(&mut stream);

    let read_temp = reader.read_u32().expect("Failed to read file");

    assert_eq!(temp, read_temp);

    cleanup("u32");
}

#[test]
fn read_write_test_u16() {
    let temp: u16 = 50;
    let mut stream = create_writer_stream("u16");
    let mut writer = BinaryWriter::new(&mut stream);

    writer.write_u16(temp).expect("Failed to write u16");

    let mut stream = create_reader_stream("u16");
    let mut reader = BinaryReader::new(&mut stream);

    let read_temp = reader.read_u16().expect("Failed to read file");

    assert_eq!(temp, read_temp);

    cleanup("u16");
}

#[test]
fn read_write_test_u8() {
    let temp: u8 = 50;
    let mut stream = create_writer_stream("u8");
    let mut writer = BinaryWriter::new(&mut stream);

    writer.write_u8(temp).expect("Failed to write u8");

    let mut stream = create_reader_stream("u8");
    let mut reader = BinaryReader::new(&mut stream);

    let read_temp = reader.read_u8().expect("Failed to read file");

    assert_eq!(temp, read_temp);

    cleanup("u8");
}

#[test]
fn read_write_bytes() {
    let count = 20;

    let temp = vec![16; count];

    let mut stream = create_writer_stream("bytes");
    let mut writer = BinaryWriter::new(&mut stream);

    writer
        .write_bytes(temp.clone())
        .expect("Failed to write bytes");

    let mut stream = create_reader_stream("bytes");

    let mut reader = BinaryReader::new(&mut stream);

    let read_temp = reader.read_bytes(count).expect("Failed to read file");

    assert_eq!(temp, read_temp);

    cleanup("bytes");
}

#[test]
#[should_panic]
fn read_out_of_range() {
    let mut stream = create_writer_stream("out_of_range");
    let mut writer = BinaryWriter::new(&mut stream);

    writer.write_f32(5.0).expect("Failed to write f32");

    let mut stream = create_reader_stream("out_of_range");

    let mut reader = BinaryReader::new(&mut stream);

    if reader.read_f32().is_err() {
        return;
    }

    println!("read_out_of_range: {}", reader.get_cur_pos().unwrap());

    if reader.read_f32().is_err() {
        cleanup("out_of_range");
        panic!("Out of range");
    }
}

#[test]
fn read_write_string() {
    let temp = "Hello World";
    let mut stream = create_writer_stream("read_write_string");
    let mut writer = BinaryWriter::new(&mut stream);

    writer
        .write_string(temp.to_string())
        .expect("Failed to write string");
    let mut stream = create_reader_stream("read_write_string");

    let mut reader = BinaryReader::new(&mut stream);
    let string = reader.read_string().expect("Failed to read string");
    assert_eq!(temp, string);

    cleanup("read_write_string");
}

#[test]
fn read_write_test_bool() {
    let positive = true;
    let negative = false;
    let mut stream = create_writer_stream("bool");
    let mut writer = BinaryWriter::new(&mut stream);

    writer.write_bool(positive).expect("Failed to write bool");
    writer.write_bool(negative).expect("Failed to write bool");

    let mut stream = create_reader_stream("bool");
    let mut reader = BinaryReader::new(&mut stream);

    let read_positive = reader.read_bool().expect("Failed to read bool");
    let read_negative = reader.read_bool().expect("Failed to read bool");

    assert_eq!(positive, read_positive);
    assert_eq!(negative, read_negative);

    cleanup("bool")
}

#[test]
fn read_write_from_memorystream() {
    let value_a = 3.0;
    let value_b = 5.0;
    let mut stream = Memorystream::new().expect("Error");
    let mut writer = BinaryWriter::new(&mut stream);
    writer.write_f32(value_a).expect("Failed to write f32");
    writer.write_f32(value_b).expect("Failed to write f32");

    let mut reader = BinaryReader::new(&mut stream);
    reader.seek_to(0).expect("Failed to seek");
    let value = reader.read_f32().expect("Failed to read f32");
    assert_eq!(value_a, value);
    let value = reader.read_f32().expect("Failed to read f32");
    assert_eq!(value_b, value);
}

#[test]
fn write_to_memorystream_overlapping() {
    let mut stream = Memorystream::new().expect("Error");
    let mut writer = BinaryWriter::new(&mut stream);
    writer.write_f32(1.0).expect("Failed to write f32");
    writer.write_f32(2.0).expect("Failed to write f32");
    writer.write_f32(3.0).expect("Failed to write f32");

    writer.seek_to(0).expect("Failed to seek to 0");
    writer.write_f32(4.0).expect("Failed to overwrite f32");
    writer.write_f32(5.0).expect("Failed to overwrite f32");
    writer.write_f32(6.0).expect("Failed to overwrite f32");

    let mut reader = BinaryReader::new(&mut stream);
    reader.seek_to(0).expect("Failed to seek");
    let value = reader.read_f32().expect("Failed to read f32");
    assert_eq!(4.0, value);
    let value = reader.read_f32().expect("Failed to read f32");
    assert_eq!(5.0, value);
    let value = reader.read_f32().expect("Failed to read f32");
    assert_eq!(6.0, value);
}

#[test]
fn write_to_memorystream_into_vec() {
    let mut stream = Memorystream::new().expect("Error");
    let mut writer = BinaryWriter::new(&mut stream);
    writer.write_f32(1.0).expect("Failed to write f32");
    let vec: Vec<u8> = stream.into();
    assert_eq!(4, vec.len());
}

#[test]
fn write_to_filestream_overlapping() {
    let mut stream = create_writer_stream("filestream_overlapping");
    let mut writer = BinaryWriter::new(&mut stream);
    writer.write_f32(1.0).expect("Failed to write f32");
    writer.write_f32(2.0).expect("Failed to write f32");
    writer.write_f32(3.0).expect("Failed to write f32");

    writer.seek_to(0).expect("Failed to seek to 0");
    writer.write_f32(4.0).expect("Failed to overwrite f32");
    writer.write_f32(5.0).expect("Failed to overwrite f32");
    writer.write_f32(6.0).expect("Failed to overwrite f32");

    let file = std::fs::File::open("filestream_overlapping.test").unwrap();
    eprintln!("File size is {}", file.metadata().unwrap().len());

    let mut stream = create_reader_stream("filestream_overlapping");
    let mut reader = BinaryReader::new(&mut stream);
    let value = reader.read_f32().expect("Failed to read f32");
    assert_eq!(4.0, value);
    let value = reader.read_f32().expect("Failed to read f32");
    assert_eq!(5.0, value);
    let value = reader.read_f32().expect("Failed to read f32");
    assert_eq!(6.0, value);

    cleanup("filestream_overlapping");
}
