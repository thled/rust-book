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
