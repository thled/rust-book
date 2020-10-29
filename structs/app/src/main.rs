fn main() {
    let user1 = User {
        mail: String::from("me@example.com"),
        name: String::from("me"),
        active: true,
        count: 1,
    };
    println!(
        "My name is '{}' with an mail of '{}' logged in {} times and am active {}.",
        user1.name,
        user1.mail,
        user1.count,
        user1.active,
    );

    let mut user2 = build_user("you@example.com".to_string(), "you".to_string());
    user2.name = String::from("he");
    println!(
        "My name is '{}' with an mail of '{}' logged in {} times and am active {}.",
        user2.name,
        user2.mail,
        user2.count,
        user2.active,
    );

    let user3 = User {
        mail: String::from("copy@example.com"),
        name: String::from("she"),
        ..user1
    };
    println!(
        "My name is '{}' with an mail of '{}' logged in {} times and am active {}.",
        user3.name,
        user3.mail,
        user3.count,
        user3.active,
    );

    let black = Color(0, 0, 0);
    println!("The color black as tuple struct: {}, {}, {}", black.0, black.1, black.2);
    let origin = Point(0, 0, 0);
    println!("The origin point as tuple struct: {}/{}/{}", origin.0, origin.1, origin.2);
}

struct User {
    name: String,
    mail: String,
    count: u64,
    active: bool,
}

fn build_user(mail: String, name: String) -> User {
    User {
        mail,
        name,
        active: true,
        count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
