use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    // There is no null in Rust
    // To account for absent values, use Option wrapper
    query_string: Option<String>,
    method: Method,
}

// to convert bytes to Request we may want to do that:
// impl Request {
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
// }

// But we actually have other constructions from the standard library
// More idiomatic rust
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        // If not implement and want to add a WIP, use the unimplemented! macro
        // unimplemented!()

        // match str::from_utf8(buffer) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // another common pattern for error handling is this one
        // match str::from_utf8(buffer).or(Err(ParseError::InvalidEncoding)){
        //     Ok(request) => {};
        //     Err(e) => return Err(e),
        // }

        // This pattern of error handling is common, so a shortcut is just:
        // let request = str::from_utf8(buffer).or(Err(ParseError::InvalidEncoding))?;

        // Actually, since the type of Error is set into the object, the default error will
        // be converted to a ParseError using the .from_type implementation, so we can just do:
        let request = str::from_utf8(buffer)?;

        unimplemented!()
    }

    // Note: the compiler will implement TryInto based on this implementation for the byte slice type
    // (reciprocal property)
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
