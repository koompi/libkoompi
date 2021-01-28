use std::fs::{self, File};
use std::io::{prelude::*, BufRead, Result, Lines, BufReader, ErrorKind, Error};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
   let file = File::open(filename)?;
   Ok(BufReader::new(file).lines())
}

pub fn read_content<P>(filename: P) -> Result<String>
where P: AsRef<Path> {
   Ok(fs::read_to_string(filename)?)
}

pub fn write_content<P>(filename: P, content: &str) -> Result<()>
where P: AsRef<Path> {
   if filename.as_ref().exists() {
      Ok(())
   } else {
      let mut file = File::create(filename)?;
      file.write_all(content.as_bytes())?;
      Ok(())
   }
}