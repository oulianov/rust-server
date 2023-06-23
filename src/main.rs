fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    // Usually, the values of a struct will be placed next to each other in memory
    // but the compiler may move things around, so don't assume any memory layout
    address: String,
}

// Implementation blocks hold the functionalities of "objects"
// The struct only hold the data
impl Server {
    // no need to add return, the value of the last line will be returned
    // naming this method 'new' is a convention
    fn new(address: String) -> Self {
        // can be written -> Server
        Server { address: address }
    }

    // self is a special argument refering
    // methods follow the ownership rules
    // fn run(&mut self: Server, ){
    fn run(self) {
        println!("Listening on {}", self.address)
    }
}

struct Request {
    path: String,
    // There is no null in Rust
    // To account for absent values, use Option wrapper
    query_string: Option<String>,
    method: Method,
}

enum Method {
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
