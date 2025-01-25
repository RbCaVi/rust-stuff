// crismas

fn main() {
    // i didn't put the rest of the lines :(
    let lines = [
        "a partridge in a pear tree",
        "two turtle doves, and ",
        "three french hens, ",
        "four calling birds, ",
    ];
    
    let ordinals = [
        "first",
        "second",
        "third",
        "fourth",
    ];
    
    for verse in 0..4 {
        print!("On the {} day of Christmas, my true love gave to me, ", ordinals[verse]);
        for line in (0..verse + 1).rev() {
            print!("{}", lines[line]);
        }
        println!("");
    }
}