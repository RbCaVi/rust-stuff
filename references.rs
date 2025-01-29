// oh no! `x` does not live long enough
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    } // x dies here, so r cannot be used after this :(

    println!("r: {r}"); // no
}

// consider:
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+

// returning a reference - lifetime generics
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

// this doesn't work :(
// needs explicit lifetimes - "missing lifetime specifier"
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// how do you use lifetimes you may ask?
// so first you add a lifetime generic parameter - ' + identifier - like 'a
// then you put it in the type after & and before mut (if it is included)
// so &'a str or &'a mut str
// a lifetime in the output is guaranteed to live at least as long as any inputs with a corresponding lifetime parameter
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// this makes the output of this function live as long as the shorter lived of its arguments

// so this works
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}

// but this doesn't (because the compiler can't tell that result is from string1)
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
}

// example: you don't always need to specify lifetime parameters on all arguments
// if one cannot be taken to use as output then it doesn't need a lifetime parameter
// why is there code based type inference but not code based lifetime inference ???????
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// consider:
// you cannot return a reference to a local variable (it is died at the end of the function) - cannot return value referencing local variable `result`
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

// oo structs with references
// every reference needs a lifetime parameter
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago..."); // call me ishmael. (moby dick? idk)
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// however there are three rules of lifetimes
// implicit lifetimes based on the method signature

// 1. every input reference gets a lifetime
// 2. if there is only one lifetime, it gets applied to all reference outputs
// 3. the self reference lifetime is applied to all reference outputs

// so this signature becomes
// fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// impl blocks must contain lifetime parameters of the struct
// (they cannot be obtained otherwise (except 'static maybe? (i don't know)))
impl<'a> ImportantExcerpt<'a> {
    // the first rule adds an implicit lifetime
    // so an extra lifetime annotation is not needed
    fn level(&self) -> i32 {
        3
    }
    
    // third rule says that self's lifetime is applied to the return value
    // and the first rule of course adds implicit lifetimes
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

// the 'static lifetime means the reference is never killed
// you might see it suggested in error messages
// but you probably want to fix reference issues like dangling reference and mismatched lifetimes

// incredible (🤯🤯)
use std::fmt::Display;

// lifetimes AND generic AND trait bound!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

