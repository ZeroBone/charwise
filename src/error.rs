use std::io::Error;

pub enum CharwiseError {
    IOError(Error)
}