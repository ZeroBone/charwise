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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
