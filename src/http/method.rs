pub enum Method {
    // In memory, represented by a number
    GET(String),
    DELETE,
    // To skip some numbers:
    // POST = 5,
    // And following items would increment from 5
    POST,
    PUT,
    HEAD,
    // We can store different value types for every enum
    // GET(String),
    // DELETE(u64),
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
