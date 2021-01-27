use std::fs::{self, File};
use std::io::{BufRead, Result, Lines, BufReader};
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