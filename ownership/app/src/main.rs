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

    let r1 = String::from("hello by ref");
    let len = calculate_length(&r1);
    println!("The length of '{}' is {}.", r1, len);

    let mut m1 = String::from("change hello");
    change(&mut m1);
    println!("{}", m1);

    let mut data_race = String::from("hello");
    let _racer1 = &mut data_race;
    // can have only one mutable ref
    // let racer2 = &mut data_race;
    // println!("{}, {}", racer1, racer2);
    
    // let mut immumut = String::from("hello");
    // let immu_ref = &immumut;
    // cannot have mut ref while having immut ref
    // let mut_ref = &mut immumut;
    // println!("{}, {}", immu_ref, mut_ref);
    
    let mut immumut = String::from("hello immutmut");
    let immu_ref = &immumut;
    println!("{}", immu_ref);
    // works because ref goes out of scope after last usage
    let mut_ref = &mut immumut;
    println!("{}", mut_ref);
    
    let hello_world = String::from("hello world");
    let hello_slice = &hello_world[0..5];
    let world_slice = &hello_world[6..11];
    println!("Slice 1 is '{}' and slice 2 is '{}'", hello_slice, world_slice);

    let two_words = String::from("firstword secondword");
    let first_word = get_first_word(&two_words);
    println!("The first word is: {}", first_word);
    let second_word = get_second_word(&two_words);
    println!("The second word is: {}", second_word);
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

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) -> () {
    s.push_str(" to this");
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn get_second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    let mut start = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            if start != 0 {
                return &s[start..i];
            }
            start = i;
        }
    }

    &s[start..]
}
