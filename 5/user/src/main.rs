struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut jdkfx = User {
        email: String::from("example@example.com"),
        username: String::from("jdkfx"),
        active: true,
        sign_in_count: 1,
    };

    println!("jdkfxのemail: {}", jdkfx.email);
    jdkfx.email = String::from("user1@example.com");
    println!("jdkfxのemail: {}", jdkfx.email);

    let hoge = build_user("hoge@example.com".to_string(), "hoge".to_string());

    let fuga = User {
        email: String::from("fuga@example.com"),
        username: String::from("fuga"),
        // active: jdkfx.active,
        // sign_in_count: jdkfx.sign_in_count,
        ..jdkfx
    };

    println!("fugaのactive: {}", jdkfx.active);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
