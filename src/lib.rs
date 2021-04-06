mod bufferize;

use std::fs::File;
use std::io::{Result, BufReader, stdin, Stdin};
use crate::bufferize::Bufferize;

pub struct Charwise<S: Bufferize> {
    source: S,
    buffer: Vec<char>,
    position: usize,
    position_in_buffer: usize
}

const CLEANUP_THRESHOLD: usize = 1024;

impl Charwise<BufReader<File>> {

    pub fn from_file(file: File) -> Self {
        Self {
            source: BufReader::new(file),
            buffer: vec![],
            position: 0,
            position_in_buffer: 0
        }
    }

}

impl Charwise<Stdin> {

    pub fn from_stdin() -> Self {
        Self {
            source: stdin(),
            buffer: vec![],
            position: 0,
            position_in_buffer: 0
        }
    }

}

impl<S: Bufferize> Charwise<S> {

    /// Returns the position of the next character to be read, or,
    /// in other words, the amount of characters read so far
    pub fn reading_position(&self) -> usize {
        self.position + self.position_in_buffer
    }

    /// Reads the next character without changing the current position
    pub fn peek(&mut self) -> Option<Result<char>> {
        self.peek_nth(0)
    }

    /// Reads the n-th character ahead of the reader without altering the current position,
    /// calling `peek_nth(0)` is equivalent to reading the next character similar to `next()`
    pub fn peek_nth(&mut self, n: usize) -> Option<Result<char>> {

        loop {

            self.cleanup_buffer();

            if self.position_in_buffer + n < self.buffer.len() {
                return Some(Ok(self.buffer[self.position_in_buffer + n]));
            }

            let mut temp_buffer = String::new();

            match self.source.read_to_string(&mut temp_buffer) {
                Ok(bytes_read) => {

                    if bytes_read == 0 {
                        // eof reached
                        return None
                    }

                    let temp_buffer: &mut Vec<char> = &mut temp_buffer.chars().collect();

                    debug_assert!(temp_buffer.len() >= 1);

                    self.buffer.append(temp_buffer);

                    debug_assert!(self.buffer.len() >= 1);

                }
                Err(e) => {
                    return Some(Err(e));
                }
            }
        }
    }

    /// Assuming the character has been peeked, advance the stream without looking at it
    /// Call this function only in case you just want to skip the current character because
    /// you already know it after calling `peek`.
    pub fn skip_peeked(&mut self) {

        debug_assert!(self.position_in_buffer < self.buffer.len());

        self.position_in_buffer += 1;
        self.cleanup_buffer();

    }

    /// Similar to `skip_peeked`, this function should be called only after calling
    /// `peek(k)` for `k >= n`. In other words, the function expects that at least n
    /// characters are already buffered and assumes that without further checks.
    pub fn advance_by(&mut self, n: usize) {

        debug_assert!(self.position_in_buffer + n < self.buffer.len());

        self.position_in_buffer += n;
        self.cleanup_buffer();

    }

    fn cleanup_buffer(&mut self) {
        if self.position_in_buffer >= CLEANUP_THRESHOLD {
            self.buffer.drain(..self.position_in_buffer);
            self.position += self.position_in_buffer;
            self.position_in_buffer = 0;
        }
    }

}

impl<S: Bufferize> Iterator for Charwise<S> {

    type Item = Result<char>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.position_in_buffer < self.buffer.len() {
            let c = self.buffer[self.position_in_buffer];
            self.position_in_buffer += 1;
            self.cleanup_buffer();
            return Some(Ok(c));
        }

        let mut temp_buffer = String::new();

        match self.source.read_to_string(&mut temp_buffer) {
            Ok(bytes_read) => {

                if bytes_read == 0 {
                    // eof reached
                    return None
                }

                self.buffer = temp_buffer.chars().collect();

                debug_assert!(self.buffer.len() >= 1);

                self.position += self.position_in_buffer;

                self.position_in_buffer = 1;

                Some(Ok(self.buffer[0]))
            }
            Err(e) => {
                Some(Err(e))
            }
        }
    }

}

#[cfg(test)]
mod tests;