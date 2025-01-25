// temperature converter

use std::io;

fn main() {
    println!("Convert from Celsius (C) or Farenheit (F)?");
    
    let system = loop {
        let mut system = String::new();
    
        io::stdin()
            .read_line(&mut system)
            .expect("Failed to read line.");
    
        match system.trim() {
            "C" => break "C",
            "F" => break "F",
            _ => {
                println!("Please enter either C or F.");
                continue;
            },
        };
    };

    let temp = loop {
        println!("Please input a temperature.");
    
        let mut temp = String::new();
    
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line.");
    
        match temp.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(err) => {
                println!("Please enter a number. ({})", err);
                continue;
            },
        }
    };
    
    if system == "C" {
        println!("{} C = {} F", temp, temp * 1.8 + 32.);
    } else {
        println!("{} F = {} C", temp, (temp - 32.) / 1.8);
    }
}