use std::io::{BufReader, Result, BufRead, Stdin};
use std::fs::File;

pub trait Bufferize {

    fn read_to_string(&mut self, target: &mut String) -> Result<usize>;

}

impl Bufferize for BufReader<File> {

    fn read_to_string(&mut self, target: &mut String) -> Result<usize> {
        self.read_line(target)
    }

}

impl Bufferize for Stdin {

    fn read_to_string(&mut self, target: &mut String) -> Result<usize> {
        self.read_line(target)
    }

}