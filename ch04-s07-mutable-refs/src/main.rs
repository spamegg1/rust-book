fn main() {
    let mut s = String::from("hello");
    change(&mut s);

    // let s1 = &mut s;
    // let s2 = &mut s; // error: cannot borrow more than once
    // println!("{s1} {s2}");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can borrow again below.
    let r2 = &mut s; // OK!
    println!("{r2}");

    let t1 = &s; // borrow as immutable ref
    let t2 = &s; // borrow as immutable ref, OK!
    println!("{t1} {t2}");

    let t3 = &mut s; // ERROR! cannot borrow as both mutable and immutable

    // println!("{t1} {t2} {t3}"); // ERROR! combine mut and immut. t1,t2 still in scope
    println!("{t3}"); // this is OK! does not combine mut and immut. t1,t2 not in scope!
}

fn change(some_string: &mut String) -> () {
    some_string.push_str(", world");
}
