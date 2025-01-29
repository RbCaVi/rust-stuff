fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}

// but consider: code deduplication by adding a function (🤯)
// this is a form of abstraction that most programmers will be familiar with
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {result}");
}

// but what if you have two functions that do the same thing
// but they use different types?????
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}

// g e n e r i c s
// this part doesn't work yet
// it says "binary operation `>` cannot be applied to type `&T`"
// you find out later that the way to solve this is a "trait bound"
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}

// also you can make generic types
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let wont_work = Point { x: 5, y: 4.0 }; // sadly, it only has one type (it should be obvious why this doesn't work)
}

// they call these templates in c++
// but here they're generics

// but you can of course have multiple generic parameters
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

// they tip that if you're using a lot of generic type parameters, then you might want to restructure your code

// also of course
// generic enums ("🤯")
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// impl blocks can be generic too
// this uses the one type point from above
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

// also you can restrict what generic instantiations get this impl
// this works with "trait bounds" too (stay tuned!)
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// a method can have generic parameters too
// even inside a generic impl block (🤯) (we all expected this (or at least i did))
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// "p e r f o r m a n c e   i s s u e s"
// they use "monomorphization"
// which is basically copy the function for every set of generic parameters it's used with
// but the computer does it so it's more accurate
// so basically no runtime cost
// maybe compile time
// but you know
// that's the tradeoff

// wait trait bounds (and traits) are in the next section

// oh well i'm stuffing it in here anyway

// so a trait is created with the trait keyword
// it's basically an abstract class in c++ (or actually java would probably fit better)
// it has the method signature followed by a semicolon
pub trait Summary {
    fn summarize(&self) -> String;
}

// now for implementation
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl trait for struct {}
// i guess if you just impl them on the struct itself it doesn't work? (haven't tested)
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// just realized rust's methods (and "classes") are different from c++
// they're structs - the functions aren't? really linked to the struct?????
// no inheritance (and with it no inheritance issues)
// i guess that does maybe discourage subclassing
// wait no you can put implementations in the trait nevermind
// then you can "extend" (implement) multiple "classes" (traits) without conflict!
// sure

// consider the following:
//use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people", // i did not know that, thanks for telling me
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// there's rules about where you can impl a trait
// you can only impl a trait if at least one of
// the trait and the struct are defined in the same "crate"
// it's from something called "coherence"
// and specifically the "orphan rule"

// default implementation
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// you still have to say that NewsArticle implements Summary
impl Summary for NewsArticle {}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    // and now it has .summarize()
    println!("New article available! {}", article.summarize());
}

// but consider: the default implementation calls a non defaulted method (🤯)
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// apparently you can't call the default implementation from an "override"

// you can use traits as parameters of functions
// it's like a hidden generic
// also in the return type, but only if the return type is unambiguous from the input types
// (so like no returning different types by an if statement or something)
// if you want that then you need a dyn (whatever that is)
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// oOoOoOo trait bound
// it's a generic
// but it has an extra restriction
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// if you use 2 impl trait parameters, then their types are not constrained
// except that they both implement the trait (they are not forced to match)
// if you want them to match, you have to write an explicit generic

// also you might be wondering how to make sure that a parameter has several traits
// just use +
// like &(impl Summary + Display) or <T: Summary + Display>

// also you can write the trait bounds after the function with where
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {1}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{1}

// return a trait (mentioned above)
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// trait bounds can be used on impl blocks too
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// apparently on struct definitions too
struct Pair<T: PartialOrd> {
    x: T,
    y: T,
}

// logically this must exist
// it's called a blanket implementation
// if T has one trait, it gets another
impl<T: Display> ToString for T {
    // --snip--
}

