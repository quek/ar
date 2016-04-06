use std::error;
use std::fmt;
use std::convert::From;

use mysql;

#[derive(Debug)]
pub enum Error {
    MySql(mysql::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::MySql(ref e) => write!(f, "MySql error: {}", e),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::MySql(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::MySql(ref e) => Some(e),
        }
    }
}

impl From<mysql::Error> for Error {
    fn from(e: mysql::Error) -> Error {
        Error::MySql(e)
    }
}
