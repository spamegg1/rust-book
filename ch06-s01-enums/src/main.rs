enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind) -> IpAddrKind {
    ip_kind
}

// easier than struct
enum IpAddr2 {
    V4(u32, u32, u32, u32),
    V6(String),
}

enum Message {
    Quit,                       // value
    Move { x: i32, y: i32 },    // struct
    Write(String),              // function
    ChangeColor(i32, i32, i32), // function
}

impl Message {
    fn call(&self) {
        println!("message!")
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddr2::V4(127, 0, 0, 1);
    let loopback2 = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // option enum
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // error
}
