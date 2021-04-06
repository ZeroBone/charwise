use std::io::{Write, Seek, SeekFrom};
use crate::Charwise;
use std::fs::File;

fn create_temp_file(data: &str) -> File {

    let mut temp_file = tempfile::tempfile().unwrap();

    temp_file.write_all(data.as_bytes()).unwrap();
    temp_file.seek(SeekFrom::Start(0)).unwrap();

    temp_file
}

fn test_file_with_data_normal(data: &str) {

    let mut cwf = Charwise::from_file(create_temp_file(data));

    for (i, c) in data.chars().enumerate() {
        assert_eq!(cwf.reading_position(), i);
        assert_eq!(cwf.peek().unwrap().unwrap(), c);
        assert_eq!(cwf.next().unwrap().unwrap(), c);
    }

    assert!(cwf.peek().is_none());
    assert!(cwf.next().is_none());

}

fn test_file_with_data_blind(data: &str) {

    let mut cwf = Charwise::from_file(create_temp_file(data));

    for (i, c) in data.chars().enumerate() {
        assert_eq!(cwf.reading_position(), i);
        assert_eq!(cwf.peek().unwrap().unwrap(), c);
        cwf.skip_peeked();
    }

    assert!(cwf.peek().is_none());
    assert!(cwf.next().is_none());

}

fn test_file_with_data(data: &str) {
    test_file_with_data_normal(data);
    test_file_with_data_blind(data);
}

#[test]
fn file_ascii_1() {
    test_file_with_data("Hello, charwise!");
}

#[test]
fn file_ascii_2() {
    test_file_with_data("dasjklf dhaskjf hadkjfh adjkfhdakj \
        fhadkfj\nhadkfjadshfkjls hzfi");
}

#[test]
fn file_ascii_3() {
    test_file_with_data("abc\n\ndef\n\n\ngh\n\n\n\njk\n\n\n\n\n");
}

#[test]
fn file_ascii_4() {
    test_file_with_data("abc\r\n\r\ndef\n\r\n\ngh\n\n\r\n\njk\r\n\r\n\r\n\r\n\r\n");
}

#[test]
fn file_ascii_5() {
    test_file_with_data("\r\n\r\n\r\n\r\n\r\n");
}

#[test]
fn file_ascii_6() {
    test_file_with_data("\r\n");
}

#[test]
fn file_ascii_7() {
    test_file_with_data("\n");
}

#[test]
fn file_nonascii_1() {
    test_file_with_data("( ͠° ͜ʖ ͠° )");
}

#[test]
fn file_nonascii_2() {
    test_file_with_data("👍( ͠° ͜ʖ ͠° 👍)");
}

#[test]
fn file_nonascii_3() {
    test_file_with_data("제 1 조 모든 인간은 태어날 때부터 자유로우며 그 존엄과 권리에 있어 동등하다.\
         인간은 천부적으로 이성과 양심을 부여받았으며 서로 형제애의 정신으로 행동하여야 한다");
}