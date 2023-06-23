use super::method::Method;
pub struct Request {
    path: String,
    // There is no null in Rust
    // To account for absent values, use Option wrapper
    query_string: Option<String>,
    method: Method,
}
