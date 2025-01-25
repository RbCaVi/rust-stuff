enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// match is pattern matching
// it's like switch(x.type) with a tagged union
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // these parts are called arms
        // each one is of the form pattern => value
        // and they are separated by commas
        // the first pattern that matches is taken
        // and it has destructuring
        // if you use a block for the value, you don't need the comma
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // the pattern can destructure (wow!)
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // if not all cases are matched, the compiler error
    // however, you can use a catch all pattern
    // which is either just a variable name or a single underscore
    // or maybe all variables starting with underscore are ignored?
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("plus_one(Some(5)) = {six}")
    println!("plus_one(None) = {none}")
}