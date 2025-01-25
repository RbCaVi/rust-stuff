fn main() {
    // if you just make an empty vector, then you need to add an annotation
    let v: Vec<i32> = Vec::new();
    
    // rust can infer the type (of course why would it not)
    let v = vec![1, 2, 3];
    
    // any value needs mut to be modified
    // rust can see that you're putting typed data
    // into it, so it can infer the type here too (🤯🤯)
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    // there are 2 ways to index a vector
    // the first way (v[i]) will panic! on an invalid index
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // the second way (v.get(i)) returns an Option
    // which contains the value if it's inside the bounds of the vector
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // compiler error - it sees that you took an immutable reference to v[0]
    // and this needs a mutable reference (which is exclusive)
    //v.push(6);

    println!("The first element is: {first}");
    
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &mut v[0];
    
    // still a compiler error
    // mutable references are still exclusive with each other
    //v.push(6);

    println!("The first element is: {first}");

    // vector iteration with for (this rust thing is really like python but compiled and stuff)
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    
    // mutable means you can update the values
    // (wow)
    // and i suppose a reference is taken here so the vector is usable afterwards
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    
    // you can define types inside the function! (🤯🤯🤯)
    #[derive(Clone)] // gotta have this so i can row.clone()
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    let mut row2 = row.clone();
    
    // crusty ahh reverse iteration
    loop {
        let x = row.pop();
        
        if let Some(x) = x {
            match x {
                SpreadsheetCell::Int(i) => println!("Integer: {i}"),
                SpreadsheetCell::Float(f) => println!("Float: {f}"),
                SpreadsheetCell::Text(s) => println!("Text: {s}"),
            }
        } else {
            break;
        }
    }
    
    // i think i could have done it with a for loop (i could)
    row2.reverse();
    let row2 = row2; // unmute it
    for x in &row2 {
        match x {
            SpreadsheetCell::Int(i) => println!("Integer: {i}"),
            SpreadsheetCell::Float(f) => println!("Float: {f}"),
            SpreadsheetCell::Text(s) => println!("Text: {s}"),
        }
    }
    
    // vectors are (of course) handled by the "borrow checker"
    // dropped at the end of the scope and everything
    // and the borrow checker checks that references to its items have shorter lifetimes than the vector
}