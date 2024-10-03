#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // version 1
    // let width1 = 30;
    // let height1 = 50;
    // println!(
    //     "The area of the rectangle is {} pixels.",
    //     area(width1, height1)
    // );

    // version 2
    // let rect = (30, 50);
    // println!("The area of the rectangle is {} pixels.", area2(rect));

    // version 3
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The rectangle is: {:#?}", rect); // implement Debug trait to print

    // area3 does an immutable borrow of rect:
    println!("The area of the rectangle is {} pixels.", area3(&rect));
    // now main can continue using rect because it's not owned by area3.

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dims: (u32, u32)) -> u32 {
    dims.0 * dims.1
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
