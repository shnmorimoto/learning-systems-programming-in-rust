use std::io::Write;
use std::str;

#[derive(Debug)]
enum BufferError {
    IOError(std::io::Error),
    StrError(str::Utf8Error),
}

impl From<std::io::Error> for BufferError {
    fn from(error: std::io::Error) -> Self {
        BufferError::IOError(error)
    }
}

impl From<str::Utf8Error> for BufferError {
    fn from(error: str::Utf8Error) -> Self {
        BufferError::StrError(error)
    }
}

fn main() -> Result<(), BufferError> {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.write(b"bytes.Buffer example1\n")?;
    println!("{}", str::from_utf8(&buffer)?);
    buffer.write(b"bytes.Buffer example2\n")?;
    println!("{}", str::from_utf8(&buffer)?);
    Ok(())
}
