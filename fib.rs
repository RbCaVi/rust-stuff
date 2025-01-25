// nth fibonacci

use std::io;

fn fib_1(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 1;
    }
    fib_1(n - 1) + fib_1(n - 2)
}

fn fib_2_h(n: u32) -> (u32, u32) {
    if n == 0 {
        return (1, 1);
    }
    let (a, b) = fib_2_h(n - 1);
    (b, a + b)
}

fn fib_2(n: u32) -> u32 {
    fib_2_h(n).0
}

fn main() {
    println!("Please input a number.");
    
    let n = loop {
        let mut n = String::new();
    
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line.");
    
        match n.trim().parse::<u32>() {
            Ok(num) => break num,
            Err(err) => {
                println!("Please enter a number. ({})", err);
                continue;
            },
        }
    };
    
    println!("the {n}th Fibonacci number is {} or {}.", fib_1(n), fib_2(n));
}