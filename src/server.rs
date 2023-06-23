// crate means the 'root' of the folder
use crate::http::Request;
use std::io::Read;
use std::net::TcpListener;

// Having a file named server.rs is the same as having
// mod server {}

// Everything inside a module is private by default
// Add the pub keyword to make it public
pub struct Server {
    // Usually, the values of a struct will be placed next to each other in memory
    // but the compiler may move things around, so don't assume any memory layout
    address: String,
}

// Implementation blocks hold the functionalities of "objects"
// The struct only hold the data
impl Server {
    // no need to add return, the value of the last line will be returned
    // naming this method 'new' is a convention
    pub fn new(address: String) -> Self {
        // can be written -> Server
        Server { address: address }
    }

    // self is a special argument refering
    // methods follow the ownership rules
    // fn run(&mut self: Server, ){
    pub fn run(self) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();
        // Rust doesn't have exceptions. It has recoverable and unrecoverable errors
        // It enforces you to deal with errors consciously before compilation
        // Result enum --> return Enum with .Ok or .Err
        // .unwrap() returns the successful result if possible, else terminates the program

        // This works:  while true {}
        // But rust has a syntax for infinite loops
        // you can also label them and break whatever labeled loop
        // 'outer: loop {
        //     // use continue; or break;
        //     'inner: loop {
        //         break 'outer;
        //     }
        // }

        loop {
            // Simple syntax
            // let res = listener.accept();
            // if res.is_err() {
            //     continue; // skip this iteration
            // }
            // // we got a good result, we can safely unwrap
            // let (stream, address) = res.unwrap();

            // match patterns are easier to deal with enum
            match listener.accept() {
                // pass an underscore to ignore a return value (python-style)
                // Ok((stream, _)) => {
                Ok((mut stream, _)) => {
                    // a reference to an array can be passed around, and functions don't need
                    // to know the exact size of the array.
                    // a reference to an array is called a slice (similar to strings)

                    // this is how to create an array with only zeros
                    // we have to specify a value for all element inside the array
                    let mut buffer = [0; 1024];
                    // this means that we only read 1024 bytes in the request ! not good for prod.

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // from_utf8_lossy replaces invalid characters with a default one
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // Convert to slice explicitely with the as keyword
                            // Request::try_from(&buffer as &[u8]);
                            // or directly convert to byte slice by slicing the whole array
                            // Request::try_from(&buffer[..]);
                            // Another way is to use try_into on the object, and specifying in the result variable the type
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                        }
                        // :? formatter uses the debug representation instead of the display one
                        // Err(e) => println!("Failed to read from connexion: {:?}", e),
                        Err(e) => println!("Failed to read from connexion: {}", e),
                    }
                }
                Err(err) => println!("Failed to establish a connection: {}", err),
                // to catch all other variants (default case in switch case)
                // _ => println!("Default match"),
                // You can add or statements
                // Ok(s) | Err(e) => {}
            }
            // matches works with other types as well
        }

        // a tuple can contain any number of different types
        // let tup = (5, "a", listener);
    }
}
