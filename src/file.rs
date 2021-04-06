use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::error::CharwiseError;

pub struct CharwiseFile {
    reader: BufReader<File>,
    buffer: Vec<char>,
    position: usize,
    position_in_buffer: usize
}

impl CharwiseFile {

    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
            buffer: vec![],
            position: 0,
            position_in_buffer: 0
        }
    }

}

impl Iterator for CharwiseFile {

    type Item = Result<char, CharwiseError>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.position_in_buffer < self.buffer.len() {
            let c = self.buffer[self.position_in_buffer];
            self.position_in_buffer += 1;
            return Some(Ok(c));
        }

        let mut temp_buffer = String::new();

        match self.reader.read_line(&mut temp_buffer) {
            Ok(bytes_read) => {

                if bytes_read == 0 {
                    // eof reached
                    return None
                }

                self.buffer = temp_buffer.chars().collect();

                assert!(self.buffer.len() >= 1);

                self.position += self.position_in_buffer;

                self.position_in_buffer = 1;

                Some(Ok(self.buffer[0]))
            }
            Err(e) => {
                Some(Err(CharwiseError::IOError(e)))
            }
        }
    }

}