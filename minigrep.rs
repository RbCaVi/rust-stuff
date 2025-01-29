use std::env;
use std::fs;

fn main() {
    // std::env::args() gives an iterator of the command line arguments
    // .collect() makes an iterator into a collection type (like vec)
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    // i think fs::read_to_string() was mentioned in fileresult2.rs
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // wow! println!() DOES have escape sequences
    println!("With text:\n{contents}");
}

// factor out parsing command line args
fn main() {
    // std::env::args() gives an iterator of the command line arguments
    // .collect() makes an iterator into a collection type (like vec)
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    println!("Searching for {query}");
    println!("In file {file_path}");

    // i think fs::read_to_string() was mentioned in fileresult2.rs
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // wow! println!() DOES have escape sequences
    println!("With text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}

// ok this returns a tuple of related values
// you should return a struct instead
fn main() {
    // std::env::args() gives an iterator of the command line arguments
    // .collect() makes an iterator into a collection type (like vec)
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // i think fs::read_to_string() was mentioned in fileresult2.rs
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // wow! println!() DOES have escape sequences
    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    // clone because no references i guess
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}

// consider constructing it instead of using an unrelated looking function
fn main() {
    // std::env::args() gives an iterator of the command line arguments
    // .collect() makes an iterator into a collection type (like vec)
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // i think fs::read_to_string() was mentioned in fileresult2.rs
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // wow! println!() DOES have escape sequences
    println!("With text:\n{contents}");
}

impl Config {
    fn new(args: &[String]) -> Config {
        // clone because no references i guess
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

// add better error handling
impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments"); // death
        }
        
        // clone because no references i guess
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

// wait, panic!() gives a stack trace and stuff
// you don't want to show users an ugly stack trace, right?
use std::process;

fn main() {
    // std::env::args() gives an iterator of the command line arguments
    // .collect() makes an iterator into a collection type (like vec)
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // i think fs::read_to_string() was mentioned in fileresult2.rs
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // wow! println!() DOES have escape sequences
    println!("With text:\n{contents}");


impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // clone because no references i guess
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// now extract the logic
fn main() {
    // std::env::args() gives an iterator of the command line arguments
    // .collect() makes an iterator into a collection type (like vec)
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    // i think fs::read_to_string() was mentioned in fileresult2.rs
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // wow! println!() DOES have escape sequences
    println!("With text:\n{contents}");
}

// run can have errors too...
use std::error::Error;

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // i think fs::read_to_string() was mentioned in fileresult2.rs
    let contents = fs::read_to_string(config.file_path)?;

    // wow! println!() DOES have escape sequences
    println!("With text:\n{contents}");

    Ok(())
}

// you would of course want to handle the result
fn main() {
    // std::env::args() gives an iterator of the command line arguments
    // .collect() makes an iterator into a collection type (like vec)
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// move the code into a separate lib.rs file
use minigrep::Config;

fn main() {
    // std::env::args() gives an iterator of the command line arguments
    // .collect() makes an iterator into a collection type (like vec)
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}