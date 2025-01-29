fn main() {
    use std::collections::HashMap; // you can put basically anything in here (i thought this had to go outside but no)

    // it can of course type inference
    // the type is HashMap<K, V>
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // wow hashmap get
    // incredible
    // it returns an option of course
    // Option<&V> specifically
    // .copied() converts Option<&V> to Option<V> (idk why? - is it some ownership thing? - is it to match 0: i32?) - reference (pointer) -> value by copying
    // there's also cloned() which is the same but uses .clone()
    // .copy() is always a shallow copy
    // .clone() might be more complicated
    // .unwrap_or(v) returns the wrapped value or v
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // iteration
    // a
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Copy able types are copied into the hash map
    // but other types are moved (and become invalid)
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    
    // also apparently references are allowed in a hash map but there's something about lifetimes idk

    // the usual - double assignment overwrites
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // cool stuff - insert if not exists
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // .entry().or_insert() will insert the value if it's not there already and returns a reference to whatever value is there now (🤯)
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    // a slightly more advanced usage of .entry().or_insert()
    // if you have a reference you need to dereference it with *?
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

}