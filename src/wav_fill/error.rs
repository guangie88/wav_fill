use hound;
use std;
use std::fmt::Formatter;
use std::path::PathBuf;
use std::str::Utf8Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CStrConv(Utf8Error),
    IO(std::io::Error),
    Hound(hound::Error),
    SameSrcDst(PathBuf),
    ZeroSamples,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "zero_fill::Error: {}"
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Error {
        Error::CStrConv(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IO(e)
    }
}

impl From<hound::Error> for Error {
    fn from(e: hound::Error) -> Error {
        Error::Hound(e)
    }
}