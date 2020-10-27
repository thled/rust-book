fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    // cannot call s anymore, because it moved into the function
    // println!("Value of s is no longer available: {}", s);
    
    let x = 5;

    makes_copy(x);

    // can still call x, because scalar types have the Copy trait
    println!("Value of x is still: {}", x);

    let s1 = gives_ownership();
    println!("The value from the function moved here: {}", s1);

    let s2 = String::from("world");

    let s3 = takes_and_gives_back(s2);
    println!("The value of s2 was given back: {}", s3);
}

fn takes_ownership(s: String) -> () {
    println!("Value of s moved into this function: {}", s);
}

fn makes_copy(x: i32) -> () {
    println!("A copy of the value of x is in this function: {}", x);
}

fn gives_ownership() -> String {
    let s = String::from("hello");

    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}
