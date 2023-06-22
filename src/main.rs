fn main() {
    let server = Server::new("127.0.0.1:8080");
    // with this syntax, we have the error : expects String but received &str
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
    fn run(self) {}
}
