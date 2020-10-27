use std::io;

fn main() {
    let nth = get_nth_input();
    let nth_fibonacci = generate_nth_fibonacci(nth);

    println!("The nth (n={}) Fibonacci number is: {}", nth, nth_fibonacci);
}

fn get_nth_input() -> u32 {
    println!("Please input which Fibonacci number you want to calculate.");
    let mut nth = String::new();
    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line");
    nth.trim().parse().expect("Not a natural number!")
}

fn generate_nth_fibonacci(nth: u32) -> u32 {
    if nth == 0 {
        return 0;
    } else if nth == 1 {
        return 1;
    } else {
        return generate_nth_fibonacci(nth-1) + generate_nth_fibonacci(nth-2);
    }
}
