use std::str::FromStr;
use std::io;

fn getval<T: FromStr>() -> T where <T as FromStr>::Err: std::fmt::Display {
    loop {
        let mut n = String::new();
    
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line.");
    
        match n.trim().parse::<T>() {
            Ok(num) => break num,
            Err(err) => {
                println!("Please enter a number. ({})", err);
                continue;
            },
        }
    }
}

fn main() {
    let mut total = 0;
    
    for counter in 0..10 {
        println!("Input grade #{}: ", counter + 1);
        
        let grade: u32 = getval();
        total += grade;
    }
	
	let average = total as f32 / 10.;
    println!("The class average is #{:.2}", average);
}
