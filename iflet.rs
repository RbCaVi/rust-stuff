fn main() {
    // if you only use one case of the match, then you can use if let instead
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    
    // you can also include an else
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("No maximum configured.");
    }
    
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => {
            println!("No maximum configured.");
        },
    }

}