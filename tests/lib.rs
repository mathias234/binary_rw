use anyhow::Result;
use binary_rw::{Endian, BinaryReader, BinaryWriter, FileStream, MemoryStream, OpenType, SliceStream, SeekStream};

fn create_writer_stream(name: &str) -> FileStream {
    let name = format!("{}.test", name);
    FileStream::new(&name, OpenType::OpenAndCreate).expect("Failed to open stream")
}

fn create_reader_stream(name: &str) -> FileStream {
    let name = format!("{}.test", name);
    FileStream::new(&name, OpenType::Open).expect("Failed to open stream")
}

fn cleanup(name: &str) {
    let name = format!("{}.test", name);
    std::fs::remove_file(&name).expect("Failure to delete file");
}

#[test]
fn slice_test() -> Result<()> {
    let mut stream = MemoryStream::new();
    let mut writer = BinaryWriter::new(&mut stream, Endian::Big);
    writer.write_u32(42)?;
    writer.write_string("foo")?;

    let buffer: Vec<u8> = stream.into();

    let mut stream = SliceStream::new(&buffer);
    let mut reader = BinaryReader::new(&mut stream, Endian::Big);

    let value = reader.read_u32()?;
    assert_eq!(42, value);

    let value = reader.read_string()?;
    assert_eq!("foo", &value);

    Ok(())
}

#[test]
fn seek_test() -> Result<()> {
    let temp: f32 = 50.0;
    let seek_loc = 5;

    let mut stream = create_writer_stream("seek");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_bytes([16; 32].to_vec())?;
    writer.seek(seek_loc)?;
    assert_eq!(writer.tell()?, seek_loc);
    writer.write_f32(temp)?;

    let mut stream = create_reader_stream("seek");
    let mut reader = BinaryReader::new(&mut stream, Default::default());
    reader.seek(seek_loc)?;
    assert_eq!(reader.tell()?, seek_loc);
    let read_temp = reader.read_f32()?;

    assert_eq!(temp, read_temp);

    cleanup("seek");

    Ok(())
}

#[test]
fn read_write_test_f64() -> Result<()> {
    let temp: f64 = 50.0;
    let mut stream = create_writer_stream("f64");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_f64(temp)?;

    let mut stream = create_reader_stream("f64");
    let mut reader = BinaryReader::new(&mut stream, Default::default());

    let read_temp = reader.read_f64()?;

    assert_eq!(temp, read_temp);

    cleanup("f64");
    Ok(())
}

#[test]
fn read_write_test_f32() -> Result<()> {
    let temp: f32 = 50.0;
    let mut stream = create_writer_stream("f32");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_f32(temp)?;

    let mut stream = create_reader_stream("f32");
    let mut reader = BinaryReader::new(&mut stream, Default::default());

    let read_temp = reader.read_f32()?;

    assert_eq!(temp, read_temp);

    cleanup("f32");

    Ok(())
}

#[test]
fn read_write_test_isize() -> Result<()> {
    let temp: isize = 50;
    let mut stream = create_writer_stream("isize");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_isize(temp)?;

    let mut stream = create_reader_stream("isize");
    let mut reader = BinaryReader::new(&mut stream, Default::default());

    let read_temp = reader.read_isize()?;

    assert_eq!(temp, read_temp);

    cleanup("isize");

    Ok(())
}

#[test]
fn read_write_test_usize() -> Result<()> {
    let temp: usize = 50;
    let mut stream = create_writer_stream("usize");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_usize(temp)?;

    let mut stream = create_reader_stream("usize");
    let mut reader = BinaryReader::new(&mut stream, Default::default());

    let read_temp = reader.read_usize()?;
    assert_eq!(temp, read_temp);

    cleanup("usize");

    Ok(())
}

#[test]
fn read_write_test_i64() -> Result<()> {
    let temp: i64 = 50;
    let mut stream = create_writer_stream("i64");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_i64(temp)?;

    let mut stream = create_reader_stream("i64");
    let mut reader = BinaryReader::new(&mut stream, Default::default());

    let read_temp = reader.read_i64()?;

    assert_eq!(temp, read_temp);

    cleanup("i64");

    Ok(())
}

#[test]
fn read_write_test_i32() -> Result<()> {
    let temp: i32 = 50;
    let mut stream = create_writer_stream("i32");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_i32(temp)?;

    let mut stream = create_reader_stream("i32");
    let mut reader = BinaryReader::new(&mut stream, Default::default());

    let read_temp = reader.read_i32()?;

    assert_eq!(temp, read_temp);

    cleanup("i32");
    Ok(())
}

#[test]
fn read_write_test_i16() -> Result<()> {
    let temp: i16 = 50;
    let mut stream = create_writer_stream("i16");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_i16(temp)?;

    let mut stream = create_reader_stream("i16");
    let mut reader = BinaryReader::new(&mut stream, Default::default());

    let read_temp = reader.read_i16()?;

    assert_eq!(temp, read_temp);

    cleanup("i16");

    Ok(())
}

#[test]
fn read_write_test_i8() -> Result<()> {
    let temp: i8 = 50;
    let mut stream = create_writer_stream("i8");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_i8(temp)?;

    let mut stream = create_reader_stream("i8");
    let mut reader = BinaryReader::new(&mut stream, Default::default());

    let read_temp = reader.read_i8()?;

    assert_eq!(temp, read_temp);

    cleanup("i8");

    Ok(())
}

#[test]
fn read_write_test_u64() -> Result<()> {
    let temp: u64 = 50;
    let mut stream = create_writer_stream("u64");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_u64(temp)?;

    let mut stream = create_reader_stream("u64");
    let mut reader = BinaryReader::new(&mut stream, Default::default());

    let read_temp = reader.read_u64()?;

    assert_eq!(temp, read_temp);

    cleanup("u64");

    Ok(())
}

#[test]
fn read_write_test_u32() -> Result<()> {
    let temp: u32 = 50;
    let mut stream = create_writer_stream("u32");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_u32(temp)?;

    let mut stream = create_reader_stream("u32");
    let mut reader = BinaryReader::new(&mut stream, Default::default());

    let read_temp = reader.read_u32()?;

    assert_eq!(temp, read_temp);

    cleanup("u32");

    Ok(())
}

#[test]
fn read_write_test_u16() -> Result<()> {
    let temp: u16 = 50;
    let mut stream = create_writer_stream("u16");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_u16(temp)?;

    let mut stream = create_reader_stream("u16");
    let mut reader = BinaryReader::new(&mut stream, Default::default());

    let read_temp = reader.read_u16()?;

    assert_eq!(temp, read_temp);

    cleanup("u16");

    Ok(())
}

#[test]
fn read_write_test_u8() -> Result<()> {
    let temp: u8 = 50;
    let mut stream = create_writer_stream("u8");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_u8(temp)?;

    let mut stream = create_reader_stream("u8");
    let mut reader = BinaryReader::new(&mut stream, Default::default());

    let read_temp = reader.read_u8()?;

    assert_eq!(temp, read_temp);

    cleanup("u8");

    Ok(())
}

#[test]
fn read_write_bytes() -> Result<()> {
    let count = 20;

    let temp = vec![16; count];

    let mut stream = create_writer_stream("bytes");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_bytes(temp.clone())?;

    let mut stream = create_reader_stream("bytes");
    let mut reader = BinaryReader::new(&mut stream, Default::default());
    let read_temp = reader.read_bytes(count)?;

    assert_eq!(temp, read_temp);

    cleanup("bytes");

    Ok(())
}

#[test]
fn read_out_of_range() -> Result<()> {
    let mut stream = create_writer_stream("out_of_range");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());
    writer.write_f32(5.0)?;

    let mut stream = create_reader_stream("out_of_range");
    let mut reader = BinaryReader::new(&mut stream, Default::default());
    reader.read_f32()?;

    assert!(reader.read_f32().is_err());

    cleanup("out_of_range");
    Ok(())
}

#[test]
fn read_write_string() -> Result<()> {
    let temp = "Hello World";
    let mut stream = create_writer_stream("read_write_string");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());
    writer.write_string(temp.to_string())?;

    let mut stream = create_reader_stream("read_write_string");
    let mut reader = BinaryReader::new(&mut stream, Default::default());
    let string = reader.read_string()?;
    assert_eq!(temp, string);

    cleanup("read_write_string");
    Ok(())
}

#[test]
fn read_write_test_bool() -> Result<()> {
    let positive = true;
    let negative = false;
    let mut stream = create_writer_stream("bool");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());

    writer.write_bool(positive)?;
    writer.write_bool(negative)?;

    let mut stream = create_reader_stream("bool");
    let mut reader = BinaryReader::new(&mut stream, Default::default());

    let read_positive = reader.read_bool()?;
    let read_negative = reader.read_bool()?;

    assert_eq!(positive, read_positive);
    assert_eq!(negative, read_negative);

    cleanup("bool");
    Ok(())
}

#[test]
fn read_write_from_memorystream() -> Result<()> {
    let value_a = 3.0;
    let value_b = 5.0;
    let mut stream = MemoryStream::new();
    let mut writer = BinaryWriter::new(&mut stream, Default::default());
    writer.write_f32(value_a)?;
    writer.write_f32(value_b)?;

    let mut reader = BinaryReader::new(&mut stream, Default::default());
    reader.seek(0)?;
    let value = reader.read_f32()?;
    assert_eq!(value_a, value);
    let value = reader.read_f32()?;
    assert_eq!(value_b, value);

    Ok(())
}

#[test]
fn write_to_memorystream_overlapping() -> Result<()> {
    let mut stream = MemoryStream::new();
    let mut writer = BinaryWriter::new(&mut stream, Default::default());
    writer.write_f32(1.0)?;
    writer.write_f32(2.0)?;
    writer.write_f32(3.0)?;

    writer.seek(0)?;
    writer.write_f32(4.0)?;
    writer.write_f32(5.0)?;
    writer.write_f32(6.0)?;

    let mut reader = BinaryReader::new(&mut stream, Default::default());
    reader.seek(0)?;
    let value = reader.read_f32()?;
    assert_eq!(4.0, value);
    let value = reader.read_f32()?;
    assert_eq!(5.0, value);
    let value = reader.read_f32()?;
    assert_eq!(6.0, value);

    Ok(())
}

#[test]
fn write_to_memorystream_into_vec() -> Result<()> {
    let mut stream = MemoryStream::new();
    let mut writer = BinaryWriter::new(&mut stream, Default::default());
    writer.write_f32(1.0)?;
    let vec: Vec<u8> = stream.into();
    assert_eq!(4, vec.len());
    Ok(())
}

#[test]
fn write_to_filestream_overlapping() -> Result<()> {
    let mut stream = create_writer_stream("filestream_overlapping");
    let mut writer = BinaryWriter::new(&mut stream, Default::default());
    writer.write_f32(1.0)?;
    writer.write_f32(2.0)?;
    writer.write_f32(3.0)?;

    writer.seek(0)?;
    writer.write_f32(4.0)?;
    writer.write_f32(5.0)?;
    writer.write_f32(6.0)?;

    let file = std::fs::File::open("filestream_overlapping.test")?;
    eprintln!("File size is {}", file.metadata()?.len());

    let mut stream = create_reader_stream("filestream_overlapping");
    let mut reader = BinaryReader::new(&mut stream, Default::default());
    let value = reader.read_f32()?;
    assert_eq!(4.0, value);
    let value = reader.read_f32()?;
    assert_eq!(5.0, value);
    let value = reader.read_f32()?;
    assert_eq!(6.0, value);

    cleanup("filestream_overlapping");

    Ok(())
}
