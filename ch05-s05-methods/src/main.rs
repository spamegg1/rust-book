#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated functions that are methods (dynamic)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // associated functions that are not methods (static)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// multiple impl blocks
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}.", rect.width);
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false

    let square = Rectangle::square(20);
    println!("{} {}", square.width, square.height);
}
