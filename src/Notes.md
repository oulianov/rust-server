String data stored on the heap: len, capacity, ptr (to the beginning of the string)
Strings can grow dynamically

let address = String::from("127.0.0.1:8080");

string slice on the heap : length and pointer to where the slice begins
no need to carry copied data. They can't make the String grow.
let string_slice = &address[10..];

 /!\ this means: return everything after the 10-th byte, not character
In UTF-8, some characters take up more than 1 byte
ex: emojis can take up to 4 bytes. Can only split on a character splits
So don't use arbitrary splits in prod code, rather use .find() and split on the index provided

you can convert full strings to string slices

let string_borrow: &str = &address; 

string literals are baked into the program memory, they are not strings by default

let string_literal = "1234";

dbg!(&address);
dbg!(string_slice);