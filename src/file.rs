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

    /// Returns the position of the next character to be read, or,
    /// in other words, the amount of characters read so far
    pub fn reading_position(&self) -> usize {
        self.position + self.position_in_buffer
    }

    /// Reads the next character without changing the current position
    pub fn peek(&mut self) -> Option<Result<char, CharwiseError>> {
        self.peek_nth(0)
    }

    /// Reads the n-th character ahead of the reader without altering the current position,
    /// calling `peek_nth(0)` is equivalent to reading the next character similar to `next()`
    pub fn peek_nth(&mut self, n: usize) -> Option<Result<char, CharwiseError>> {

        loop {

            self.cleanup_buffer();

            if self.position_in_buffer + n < self.buffer.len() {
                return Some(Ok(self.buffer[self.position_in_buffer + n]));
            }

            let mut temp_buffer = String::new();

            match self.reader.read_line(&mut temp_buffer) {
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
                    return Some(Err(CharwiseError::IOError(e)));
                }
            }
        }
    }

    fn cleanup_buffer(&mut self) {
        // TODO
    }

}

impl Iterator for CharwiseFile {

    type Item = Result<char, CharwiseError>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.position_in_buffer < self.buffer.len() {
            let c = self.buffer[self.position_in_buffer];
            self.position_in_buffer += 1;
            self.cleanup_buffer();
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

                debug_assert!(self.buffer.len() >= 1);

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

#[cfg(test)]
mod tests {
    use std::io::{Write, Seek, SeekFrom};
    use crate::file::CharwiseFile;

    fn test_file_simple_iter(data: &str) {

        let mut temp_file = tempfile::tempfile().unwrap();
        temp_file.write_all(data.as_bytes()).unwrap();
        temp_file.seek(SeekFrom::Start(0)).unwrap();

        let mut cwf = CharwiseFile::new(temp_file);

        for (i, c) in data.chars().enumerate() {
            assert_eq!(cwf.reading_position(), i);
            assert_eq!(cwf.peek().unwrap().unwrap_or('\0'), c);
            assert_eq!(cwf.next().unwrap().unwrap_or('\0'), c);
        }

        assert!(cwf.peek().is_none());
        assert!(cwf.next().is_none());

    }

    #[test]
    fn file_ascii_1() {
        test_file_simple_iter("Hello, charwise!");
    }

    #[test]
    fn file_ascii_2() {
        test_file_simple_iter("dasjklf dhaskjf hadkjfh adjkfhdakj \
        fhadkfj\nhadkfjadshfkjls hzfi");
    }

    #[test]
    fn file_ascii_3() {
        test_file_simple_iter("abc\n\ndef\n\n\ngh\n\n\n\njk\n\n\n\n\n");
    }

    #[test]
    fn file_ascii_4() {
        test_file_simple_iter("abc\r\n\r\ndef\n\r\n\ngh\n\n\r\n\njk\r\n\r\n\r\n\r\n\r\n");
    }

    #[test]
    fn file_ascii_5() {
        test_file_simple_iter("\r\n\r\n\r\n\r\n\r\n");
    }

    #[test]
    fn file_ascii_6() {
        test_file_simple_iter("\r\n");
    }

    #[test]
    fn file_ascii_7() {
        test_file_simple_iter("\n");
    }

    #[test]
    fn file_nonascii_1() {
        test_file_simple_iter("( ͠° ͜ʖ ͠° )");
    }

    #[test]
    fn file_nonascii_2() {
        test_file_simple_iter("👍( ͠° ͜ʖ ͠° 👍)");
    }

    #[test]
    fn file_nonascii_3() {
        test_file_simple_iter("제 1 조 모든 인간은 태어날 때부터 자유로우며 그 존엄과 권리에 있어 동등하다.\
         인간은 천부적으로 이성과 양심을 부여받았으며 서로 형제애의 정신으로 행동하여야 한다");
    }

}