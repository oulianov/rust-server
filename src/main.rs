use http::Method;
use http::Request;
use server::Server;

mod http;
mod server; // pull the module into the scope. necessary to use it

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
