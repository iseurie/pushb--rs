extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate curl;
extern crate clap;

mod note;
mod target;

pub use self::note::*;
pub use self::target::Target;

use std::convert::From;
use std::fmt::{self, Display, Formatter};
use std::error::{self, Error as StdError};
use std::result;

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub cat: String,
    #[serde(rename = "type")]
    pub message: String,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        f.write_str(self.description())?;
        Ok(())
    }
}

impl error::Error for ApiError {
    fn description(&self) -> &str {
        &self.message
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug)]
pub enum Error {
    Curl(curl::Error),
    Api(u32, Option<ApiError>),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        match self {
            &Error::Curl(ref err) => {
                f.write_fmt(format_args!("cURL: {}", err.description()))?;
            },
            &Error::Api(status, ref opt_err) => {
                f.write_fmt(format_args!("HTTP/{}: ", status))?;
                f.write_str(
                    if let &Some(ref err) = opt_err {
                        err.description()
                    } else {
                        "<irretrievable>"
                    }
                )?;
            },
        }
        Ok(())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Curl(ref e) => e.description(),
            &Error::Api(_, ref opt_err) => {
                if let &Some(ref e) = opt_err {
                    e.description()
                } else {
                    "<irretrievable>"
                }
            },
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &Error::Curl(ref err) => Some(err),
            &Error::Api(_, ref opt_err) => {
                if let &Some(ref err) = opt_err {
                    Some(err)
                } else {
                    None
                }
            }
        }
    }
}

impl From<curl::Error> for Error {
    fn from(c: curl::Error) -> Self {
        Error::Curl(c)
    }
}

pub type Result<T> = result::Result<T, Error>;

pub trait Push {
    fn push(&self, key: &str) -> Result<()>;
}
