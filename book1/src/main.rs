struct User {
    username: String,
    password: String,
    is_active: bool,
    log_count: u32
}

struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

fn build_user(username: String, password: String) -> User {
    User {
        username,
        password,
        is_active: true,
        log_count: 0
    }
}

fn updated_user(username: String, password: String, user: &User) -> User {
    User {
        username,
        password,
        ..*user
    }
}

fn main() {
    let mut me = build_user(String::from("droox"), String::from("pass"));
    println!("{}", me.password);
    println!("{}", me.username);
    println!("{}", me.is_active);
    println!("{}", me.log_count);
    me.log_count += 1;
    println!("{}", me.log_count);

    let you = updated_user(String::from("frv3r"), String::from("pussy"), &me);
    println!("{}", me.username);
    println!("{}", you.username);

    let red = Color(255, 0, 0);
    println!("{}", red);
}

