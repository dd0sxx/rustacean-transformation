fn main() {
    
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        email: String::from("crab@example.com"),
        username: String::from("crabtastic"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("rustacean@gmail.com");

    // field init shorthand
    fn build_user(email: String, username: String) -> () {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1
        };
    }

        // struct update syntax ..
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
        // user1 no longer valid because we moved the username value into user2
    };

    //tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    //unit-like structs are empty structs
    struct AlwaysEqual;
    let subject = AlwaysEqual;

}