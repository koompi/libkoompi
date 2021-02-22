use std::fs::{self, File};
use std::io::{prelude::*, BufRead, Result, Lines, BufReader};
use std::path::Path;

/// This function is used to read file line by line and return resulf of lines.
pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
   let file = File::open(filename)?;
   Ok(BufReader::new(file).lines())
}

/// This function is used to read file and return the content.
pub fn read_content<P>(filename: P) -> Result<String>
where P: AsRef<Path> {
   Ok(fs::read_to_string(filename)?)
}

/// This function is used to write content to file if file does not exists.
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

/// This function is used to write content to file even file is exist or not.
pub fn write_content_overwrite<P>(filename: P, content: &str) -> Result<()>
where P: AsRef<Path> {
   let mut file = File::create(filename)?;
   file.write_all(content.as_bytes())?;
   Ok(())
}