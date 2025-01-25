// you can end a block with an expression without a semicolon
// that's the return value
fn five() -> i32 {
    5
}

fn six() -> i32 {6}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
    println!("The value of six is: {}", six());
    
    // ternary (wow!)
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    
    // loops return with break
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    
    // loop labels!
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    
    // while of course
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    
    // for each!
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    
    // variable shadowing makes it so i can just copy self contained bits of code
    
    // range (1..4) and reverse .rev()
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}