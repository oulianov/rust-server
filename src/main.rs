use http::request::Request;
use server::Server;

mod server; // pull the module into the scope. necessary to use it

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod http {
    // submodules also have visibilities and have to be specified public
    pub mod request {
        use super::method::Method;
        pub struct Request {
            path: String,
            // There is no null in Rust
            // To account for absent values, use Option wrapper
            query_string: Option<String>,
            method: Method,
        }
    }

    pub mod method {

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
    }
}
