fn main() {
    // apparently a string is a vector<byte> but like "better"
    let mut s = String::new();
    println!("s is {s} (it's empty)");

    // you can use .to_string() on any Display able struct
    let data = "initial contents";

    let s = data.to_string();
    println!("s is {s}");

    // the method also works on a literal directly: // this is not my comment >:(
    let s = "initial contents".to_string();
    println!("s is {s}");
    
    // string from string literal
    // probably strdup or something
    let s = String::from("initial contents");
    println!("s is {s}");

    // incredible (you can store various encodings (utf 8))
    println!("I'm going to say hello in like 10 languages!");
    let hello = String::from("السلام عليكم");
    println!("{hello}");
    let hello = String::from("Dobrý den");
    println!("{hello}");
    let hello = String::from("Hello");
    println!("{hello}");
    let hello = String::from("שלום");
    println!("{hello}");
    let hello = String::from("नमस्ते");
    println!("{hello}");
    let hello = String::from("こんにちは");
    println!("{hello}");
    let hello = String::from("안녕하세요");
    println!("{hello}");
    let hello = String::from("你好");
    println!("{hello}");
    let hello = String::from("Olá");
    println!("{hello}");
    let hello = String::from("Здравствуйте");
    println!("{hello}");
    let hello = String::from("Hola");
    println!("{hello}");

    // append string easy! (i don't have much experience with string manipulation in c++)
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {s}");

    // it takes a string slice so
    // it can take a string without killing (freeing) it
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    println!("s is {s}");

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {s}");

    // y o u   c a n   a d d   s t r i n g s! (🤯🤯🤯)
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // this uses the add() method
    println!("s3 is {s3}");
    println!("s2 is {s2}");
    
    // so could i do this?
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    use std::ops::Add;
    let s3 = s1.add(&s2); // yes i can (but apparently i have to "include" (use) std::ops::Add)
    println!("s3 is {s3}");
    println!("s2 is {s2}");
    
    // it appends the rest of the strings to the end of the first
    // so it copies less often
    // (it's like pv_string)
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {s}");

    // the format! macro (of course they have this...)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {s}");
    // it doesn't take ownership of any of the strings either
    println!("s1 is {s1}");
    println!("s2 is {s2}");
    println!("s3 is {s3}");

    // you can't use normal indexes for strings :(
    //let s1 = String::from("hello");
    //let h = s1[0];

    // you can use slices though
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("s is {s}");
    // it will however panic if you try to slice half of a character

    // you can iterate over characters and bytes
    // grapheme cluster iteration is not in the standard library :(but who cares)
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}