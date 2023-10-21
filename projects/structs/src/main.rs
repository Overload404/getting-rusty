fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("overload"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    user1.email = String::from("something@something.com"); // changing a value of an instance field

    println!("{}", user1.email);

    let user2 = User { // creating struct instance from another instance
        active: user1.active,
        username: user1.username,
        email: String::from("somethingelse@something.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("ligma@sugma.com"),
        ..user2
    };

    let mess1 = Mess(10, String::from("something"), 3.32);
}

struct User {
    active: bool,
    username: String,
    email: String, 
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1, 
    }
}

struct Mess(i32, String, f64); // Tuple struct

struct AlwaysEqual;