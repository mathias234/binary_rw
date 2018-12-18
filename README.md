# binary-rs
A binary reader/writer for the rust language, with a similar syntax to the C# BinaryWriter and BinaryReader

Example code for reading
```rust
extern crate binary_rw;
use binary_rw::{BinaryReader, OpenType};

fn main() {
  let mut binary_file = BinaryReader::new("data.dat", OpenType::OpenAndCreate);

  let read_value = binary_file.read_f32();
  println(read_value);
}
```

Example code for writing
```rust
extern crate binary_rw;
use binary_rw::{BinaryWriter, OpenType};

fn main() {
  let mut binary_file = BinaryWriter::new("data.dat", OpenType::Open);
  
  let value: f32 = 30.5;
  binary_file.write_f32(value);
}
```
