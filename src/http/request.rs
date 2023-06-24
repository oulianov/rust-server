use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request<'buf> {
    path: &'buf str,
    // There is no null in Rust
    // To account for absent values, use Option wrapper
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

// to convert bytes to Request we may want to do that:
// impl Request {
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
// }

// But we actually have other constructions from the standard library
// More idiomatic rust
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buffer: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
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

        // The naive way to implement the get_next_word iteration is like this
        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }

        // But we can also use
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        // note that we're overwriting the value of request. This is called variable shadowing
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        // implementing from_str gave use the .parse method for free
        let method: Method = method.parse()?;

        // One way to split the path is like this. We only care if we find stuff
        // let mut query_string = None;
        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        // A way to avoid None => {} line, we can do :
        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }

        // we can even make this simpler with the syntax if let
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path: path,
            query_string: query_string,
            method: method,
        })
    }

    // Note: the compiler will implement TryInto based on this implementation for the byte slice type
    // (reciprocal property)
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // A naive loop is the following
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {}
    //         None => break,
    //     }
    // }

    // Alternatively just use a for loop
    // .enumerate is similar to python
    for (i, c) in request.chars().enumerate() {
        // \r is carriage return, aka new line
        if c == ' ' || c == '\r' {
            // Note: the lower-bound is inclusive. So we can add 1 to the index
            // however, the strings are byte indexed, not character indexed. Splitting along a non-character boundary will fail.
            // Here it's ok since we are splitting over spaces
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    // Return None
    None
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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
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
