fn main() {
    let mut mut_v: Vec<u32> = Vec::new();
    mut_v.push(1);
    mut_v.push(2);
    mut_v.push(3);
    println!(
        "Vector mutV contains: {}, {}, {}",
        mut_v[0], mut_v[1], mut_v[2]
    );

    let v = vec![10, 20, 30];
    println!("Vector v contains: {}, {}, {}", v[0], v[1], v[2]);

    let second = &v[1];
    println!("Second element of v is: {}", second);

    match v.get(1) {
        Some(second) => println!("Second element via get is: {}", second),
        None => println!("There is no second element!"),
    };

    let iter_v = vec![1, 1, 2, 3, 5, 8];
    for i in &iter_v {
        println!("Fibo: {}", i);
    }

    let mut mut_iter_v = vec![1, 2, 3];
    for i in &mut mut_iter_v {
        *i += 10;
    }
    println!(
        "Vector mut_iter_v contains: {}, {}, {}",
        mut_iter_v[0], mut_iter_v[1], mut_iter_v[2]
    );

    // different types within a vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
