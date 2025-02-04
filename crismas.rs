// crismas

fn main() {
    let lines = [
        "a partridge in a pear tree",
        "two turtle doves, and ",
        "three french hens, ",
        "four calling birds, ",
        "five golden rings, ",
        "six geese a laying, ",
        "seven swans a swimming, ",
        "eight maids a milking, ",
        "nine ladies dancing, ",
        "ten lords a leaping, ",
        "eleven pipers piping, ",
        "twelve drummers drumming, ",
    ];
    
    let ordinals = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];
    
    for verse in 0..12 {
        print!("On the {} day of Christmas, my true love gave to me, ", ordinals[verse]);
        for line in (0..verse + 1).rev() {
            print!("{}", lines[line]);
        }
        println!("");
    }
}
