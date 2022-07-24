# binary-rs
[![Crate](https://img.shields.io/crates/v/binary_rw)](https://crates.io/crates/binary_rw)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/mathias234/binary-rs/Code%20coverage%20with%20grcov?label=tests)](https://github.com/mathias234/binary-rs/actions/workflows/rust.yml)
[![Coverage Status](https://coveralls.io/repos/github/mathias234/binary-rs/badge.svg?branch=master)](https://coveralls.io/github/mathias234/binary-rs?branch=master)

A binary reader/writer for the rust language, with a similar syntax to the C# BinaryWriter and BinaryReader

#### Examples

Example code for reading

```rust
extern crate binary_rw;
use binary_rw::{
    filestream::{Filestream, OpenType},
    BinaryReader
};

fn main() {
  let mut fs = Filestream::new("test.bin", OpenType::Open).expect("Failed to open file"); 
  let mut binary_file = BinaryReader::new(&mut fs);

  let read_value = binary_file.read_f32().expect("Failed to read f32");
  println!("{:?}", read_value);
}
```

Example code for writing
```rust
extern crate binary_rs;
use binary_rs::{
    filestream::{Filestream, OpenType},
    BinaryReader
};

fn main() {
  let mut fs = Filestream::new("test.bin", OpenType::OpenAndCreate).expect("Failed to open file"); 
  let mut binary_file = BinaryWriter::new(&mut fs);
  
  let value: f32 = 30.5;
  binary_file.write_f32(value).expect("Failed to write f32");
}
```

#### TODO

