// pig latin

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    
    let input = input; // unmute
    
    println!("{input}");
    
    let mut output = String::new();
    
    let mut inword = false;
    
    let mut firstchar = '\0';
    
    for c in input.chars() {
        if c.is_whitespace() {
            if inword {
                inword = false;
                output.push_str(&format!("-{firstchar}ay"));
            }
            output.push(c);
        } else {
            if !inword {
                inword = true;
                if "aeiouAEIOU".contains(c) {
                    output.push(c);
                    firstchar = 'h';
                } else {
                    firstchar = c;
                }
            } else {
                output.push(c);
            }
        }
    }
    
    println!("{output}");
}