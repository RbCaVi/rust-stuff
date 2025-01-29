use std::net::IpAddr;

fn main() {
    // if you're sure the input is valid
    // then you can use .expect() / .unwrap()
    // instead of matching
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    // there are of course criteria for using panic!()
    // a "bad state" - when the contract is broken - but also one of
    // unexpected - not something that would happen a lot
    // code downstream does not check for this state
    // "there's not a good way to encode this information"
    
    // basically panic means unrecoverable
    // result means recoverable (possibly)
}

// consider: create a custom type for validation
// the module system (which i have not read about yet)
// keeps the members of the struct private from other code i guess
pub struct Guess {
    value: i32,
}

impl Guess {
    // create a Guess from an integer.
    // the integer must be within the range of 1 to 100 inclusive
    // otherwise Guess::new() will panic
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    // return the value contained inside a Guess
    pub fn value(&self) -> i32 {
        self.value
    }
}