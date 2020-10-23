fn main() {
    // loop
    let mut counter = 0;
    
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // looping through a collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    // for loop
    for element in a.iter() {
        println!("The value through the for loop is: {}", element);
    }

    // nicer countdown
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!! (nicer)");
}
