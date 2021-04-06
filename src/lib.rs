mod file;
mod error;

use std::fs::File;
use crate::file::CharwiseFile;
use crate::error::CharwiseError;

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

}

impl Iterator for Charwise {

    type Item = Result<char, CharwiseError>;

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