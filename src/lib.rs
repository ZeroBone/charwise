pub mod file;

use std::fs::File;
use std::io::Result;
use crate::file::CharwiseFile;

pub enum Charwise {
    File(CharwiseFile),
    Stdin
}

impl Charwise {

    pub fn from_file(file: File) -> Self {
        Self::File(CharwiseFile::new(file))
    }

    /// Returns the position of the next character to be read, or,
    /// in other words, the amount of characters read so far
    pub fn reading_position(&self) -> usize {
        match self {
            Charwise::File(cf) => cf.reading_position(),
            Charwise::Stdin => todo!()
        }
    }

    pub fn peek(&mut self) -> Option<Result<char>> {
        match self {
            Charwise::File(cf) => cf.peek(),
            Charwise::Stdin => todo!()
        }
    }

}

impl Iterator for Charwise {

    type Item = Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Charwise::File(cf) => {
                cf.next()
            }
            Charwise::Stdin => {
                todo!()
            }
        }
    }

}