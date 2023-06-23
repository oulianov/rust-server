use super::method::Method;
use std::convert::TryFrom;
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
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // If not implement and want to add a WIP, use the unimplemented! macro
        unimplemented!()
    }

    // Note: the compiler will implement TryInto based on this implementation for the byte slice type
    // (reciprocal property)
}
