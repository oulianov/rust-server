fn main() {
    // String data stored on the heap: len, capacity, ptr (to the beginning of the string)
    // Strings can grow dynamically
    let address = String::from("127.0.0.1:8080");
    // string slice on the heap : length and pointer to where the slice begins
    // no need to carry copied data. They can't make the String grow.
    let string_slice = &address[10..]; // /!\ this means: return everything after the 10-th byte, not character
                                       // In UTF-8, some characters take up more than 1 byte
                                       // ex: emojis can take up to 4 bytes. Can only split on a character splits
                                       // So don't use arbitrary splits in prod code, rather use .find() and split on the index provided
    let string_borrow: &str = &address; // you can convert strings to string slices

    let string_literal = "1234";

    dbg!(&address);
    dbg!(string_slice);
    let server = Server::new("127.0.0.1:8080".to_string());
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
    fn run(self) {
        println!("Listening on {}", self.address)
    }
}
