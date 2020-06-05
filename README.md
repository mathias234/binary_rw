# binary-rs

A binary reader/writer for the rust language, with a similar syntax to the C# BinaryWriter and BinaryReader

#### Examples

Example code for reading

```rust
extern crate binary_rw;
use binary_rw::{BinaryReader, OpenType};

fn main() {
  let mut binary_file = BinaryReader::new("data.dat", OpenType::OpenAndCreate).expect("Failed to create reader");

  let read_value = binary_file.read_f32().expect("Failed to write f32");
  println!("{:?}", read_value);
}
```

Example code for writing
```rust
extern crate binary_rw;
use binary_rw::{BinaryWriter, OpenType};

fn main() {
  let mut binary_file = BinaryWriter::new("data.dat", OpenType::Open).expect("Failed to create writer");
  
  let value: f32 = 30.5;
  binary_file.write_f32(value).expect("Failed to write f32");
}
```

#### TODO

- Multiple underlying streams like Memory Stream and File Stream (only file stream currently)
