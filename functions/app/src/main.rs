fn main() {
    println!("Hello, world!");

    another_function();

    param_function(5);

    multi_param_func(7, 8);

    expressions();

    let x = nine();
    println!("The value of x is: {}", x);
}

fn nine() -> i32 {
    9
}

fn expressions() {
    let x = {
        let y = 3;
        y + 1
    };
    println!("The value of x is the return of the expression of the curly braces: {}", x);
}

fn multi_param_func(x: i32, y: i32) {
    println!("The value of x and y are: {} and {}", x, y)
}

fn param_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}
