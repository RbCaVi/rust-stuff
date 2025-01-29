use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // some functions will return Result<V, E>
    // which is like Option<V> (value or none)
    // but it's value or error
    let greeting_file_result = File::open("hello.txt");

    // this is fairly straightforward, if a bit difficult to understand
    let greeting_file = match greeting_file_result {
        Ok(file) => file, // if it worked, then return the file
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") { // if file not found, try to create it
                Ok(fc) => fc, // it worked
                Err(e) => panic!("Problem creating the file: {e:?}"), // death
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}"); // death
            }
        },
    };
    
    // this is a common problem in most or all standard
    // text based programming languages (🤯) - branching structures are difficult to represent
    // the linear nature of the code also restricts the complexity of structures
    // most of the time you probably don't need weird branching and rejoining
    // of code paths, but when you do, most languages force you to use either several
    // boolean flags to track which branch to take or goto (which usually makes code harder to understand)
    // the restriction of code to one dimension forces branching structures to have at least one branch far
    // from it's root, which makes code with lots of branches difficult to read
    
    // this is why i have an idea (🤯)
    // actually this is not the reason but my idea might solve this problem
    // but like what if you wrote programs with nodes and wires
    // in a 2d or 3d editor
    // then you could visually see (with your eyes) how the control flow works
    // there would of course be a bit less emphasis on ordering of operations
    // and (maybe?) a bit more on data flow
    
    // uhhhh
    // .unwrap_or_else() takes one of these closure things
    // one that takes the error and returns the correct type
    // (or panics or something i guess)
    // consider: Result<Result<T, E>, E> -> Result<T, E> (monad or "smth")
    // same as above
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        // the file didn't work...
        if error.kind() == ErrorKind::NotFound {
            // it didn't exist :(
            File::create("hello.txt").unwrap_or_else(|error| {
                // at least i tried :)
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
    
    // .unwrap() panics if the thing is an error
    // it returns the value inside otherwise
    let greeting_file = File::open("hello.txt").unwrap();
    
    // .expect() does the same thing but also makes a message
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // well, i tried my best
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    } // hidden return (this is something i am familiar with by now)
    // like i guess there might be reasons for and against this, but
    // it definitely makes really small functions even shorter
}

fn read_username_from_file() -> Result<String, io::Error> {
    // NEW THING!
    // thing? is an early return on error (🤯🤯🤯)
    // it can propagate Result and Option
    // and also FromResidual, whatever that is
    // if thing is Result::Err(), then it immediately returns it
    // or i guess converts it to the error type in the signature first
    // it can only be used in a function that returns a matching type
    // haha you can remove return from your vocabulary
    // by only using ? and implicit return
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    
    // you can even call more functions on it (of course)
    // this looks like js's .?
    // but it's definitely different
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file() -> Result<String, io::Error> {
    // and they have a library function just for this specific use case (🤯🤯🤯)
    // it's a thing i do a lot
    // like why doesn't js have this
    // even with python you have to use either with open() as f: or f = open(); f.close()
    // fun fact: unreadable code
    // file_contents = [[f.read(), f.close()][0] for f in [open(filename)]][0]
    fs::read_to_string("hello.txt")
}

// you can only use thing? in a function with a compatible return type
// (unrelated) functions implicitly return () (unit type)

// also
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// but you can't change between option and result automatically

// you know, rust makes it at least a bit harder to write bad or incomprehensible code
// option instead of null, no implicit cast (use as), sensible shortcuts like ?
// they use result instead of exceptions
// and i guess i can see sense in that
// actually i don't know any arguments for exceptions so

// cool - dyn Error (it's a "trait object", whatever that means)
// this, however, does return a result, so you can use ?
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
// apparently main can return any type with std::process::Termination
// which has report() -> ExitCode