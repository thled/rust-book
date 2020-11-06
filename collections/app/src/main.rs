use std::collections::HashMap;

fn main() {
    // Vectors
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

    // Strings
    let mut string = String::new();

    let data = "initial contents";
    let s_data = data.to_string();
    let s_data_direct = "initial contents".to_string();
    let s_from = String::from("initial contents");

    let mut foobar = String::from("foo");
    foobar.push_str("bar");

    let mut lol = String::from("lo");
    lol.push('l');

    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let hello_world = hello + &world;
    println!("{}", hello_world);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    // let tic_tac_toe = tic + "-" + &tac + "-" + &toe;
    // println!("{}", tic_tac_toe);

    let tic_tac_toe_format = format!("{}-{}-{}", tic, tac, toe);
    println!("{}", tic_tac_toe_format);

    let hello = "Здравствуйте";
    let four_bytes_slice = &hello[0..4];
    println!("A 0..4 range returns {} from {}.", four_bytes_slice, hello);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Hashmaps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let init_scores = vec![10, 50];
    let mut scores_collect: HashMap<_, _> =
        teams.into_iter().zip(init_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut color_map = HashMap::new();
    color_map.insert(field_name, field_value);
    // println!(
    //     "field_name {} and field_value {} are not available.",
    //     field_name, field_value
    // );

    let score = scores.get("Blue");
    match score {
        Some(&team_score) => println!("Blue team's score is: {}", team_score),
        None => println!("Blue team has no score!"),
    };

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Green"), 10);
    scores.insert(String::from("Green"), 25);
    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
