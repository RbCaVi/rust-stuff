// list / vector -> median & mode

use std::io;
use std::collections::HashMap;

fn main() {
    println!("Please input a list of numbers separated by spaces.");
    
    let mut ns = 'ns: loop {
        let mut ns = String::new();
    
        io::stdin()
            .read_line(&mut ns)
            .expect("Failed to read line.");
        
        let mut nsv = Vec::new();
        for n in ns.trim().split_whitespace() {
            match n.parse::<u32>() {
                Ok(num) => nsv.push(num),
                Err(err) => {
                    println!("Please enter a proper list of numbers. ({})", err);
                    continue 'ns;
                },
            }
        }
        
        break nsv;
    };
    
    println!("You entered {:?}", ns);
    
    ns.sort();
    
    println!("Sorted: {:?}", ns);
    
    let len = ns.len();
    
    if len % 2 == 0 {
        let i2 = len / 2;
        let i1 = i2 - 1;
        let n1 = ns[i1];
        let n2 = ns[i2];
        println!("The median of the list is {}", (n1 + n2) as f64 / 2.);
    } else {
        let i = len / 2;
        let n = ns[i];
        println!("The median of the list is {n}");
    }
    
    let mut counts = HashMap::new();
    
    for n in ns {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
    }
    
    let mut maxcount = 0;
    let mut modes = Vec::new();
    
    for (n, count) in counts {
        if count >= maxcount {
            if count > maxcount {
                modes = Vec::new();
                maxcount = count;
            }
            modes.push(n);
        }
    }
    
    println!("The mode(s) are: {:?}", modes);
}