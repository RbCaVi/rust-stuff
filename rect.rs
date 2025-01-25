// rust people like to pass a struct instead of 2 related values or a tuple
// it's not clear that the 2 arguments are related
// and tuple indexing is a bit confusing

#[derive(Debug)] // allow printing this with {:?} or {:#?} format in println!()
struct Rectangle {
    width: u32,
    height: u32,
}

// methods are defined in a separate impl block
// the self parameter is required
// self: Self (possibly with & or &mut)
// but the : Self part is implicit (aren't these rust people so nice?)
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // you can name a method the same as a member 🤯🤯🤯🤯
    fn width(&self) -> bool {
        self.width > 0
    }
}

// also you can have multiple impl blocks for one struct!
impl Rectangle {
    // you can't have this as a method :(
    //fn can_hold(this: &Rectangle, that: &Rectangle) -> bool {
    //    this.width > that.width && this.height > that.height
    //}
    
    // the first parameter name is locked to be self if you want it to be a method
    fn can_hold(&self, that: &Rectangle) -> bool {
        self.width > that.width && self.height > that.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!(
        "The area of the rectangle is also {} square pixels.",
        rect1.area()
    );

    println!(
        "The rectangle has a nonzero width: {}",
        rect1.width()
    );

    // you can also call methods off their class like in python
    // it needs explicit borrowing though
    // (dot notation does implicit & or &mut)
    println!(
        "The rectangle has a nonzero width: {}",
        Rectangle::width(&rect1)
    );
    
    let rect2 = Rectangle {
        width: 20,
        height: 20,
    };
    
    println!(
        "rect1 can hold rect2: {}",
        rect1.can_hold(&rect2)
    );

    dbg!(&rect1);

    //rect1 = dbg!(rect1); // the macro takes ownership of rect1 and i can't take it back by reassignment because rect1 is immutable

    dbg!(rect1);
    
    // so now i can't use rect1 anymore
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}