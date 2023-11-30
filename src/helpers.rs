use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

/// Read a file into a Result<Vec<String>>
pub fn read_input(path: &Path) -> Result<Vec<String>, Error> {
    BufReader::new(File::open(path)?)
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()
}