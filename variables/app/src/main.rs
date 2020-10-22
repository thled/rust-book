const VALUE_CANNOT_BE_DYNAMIC: u32 = 100_000;

fn main() {
    println!("The value of the constant is: {}", VALUE_CANNOT_BE_DYNAMIC);

    let mut x_mutable = 5;
    println!("The value of x_mutable is: {}", x_mutable);
    x_mutable = 6;
    println!("The value of x_mutable is: {}", x_mutable);

    let x_shadowed = 5;
    println!("The value of x_shadowd is: {}", x_shadowed);
    let x_shadowed = 6;
    println!("The value of x_shadowd is: {}", x_shadowed);
}
