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
                Ok((stream, address)) => {
                    let a = 5;
                    println!("OK!")
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
