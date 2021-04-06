use std::io::{Write, Seek, SeekFrom};
use crate::Charwise;

fn test_file_simple_iter(data: &str) {

    let mut temp_file = tempfile::tempfile().unwrap();
    temp_file.write_all(data.as_bytes()).unwrap();
    temp_file.seek(SeekFrom::Start(0)).unwrap();

    let mut cwf = Charwise::from_file(temp_file);

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