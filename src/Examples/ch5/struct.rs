struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let mut user1 : User = User {
        email: String::from("email@gmail.com"),
        username : String::from("username"),
        active : true,
        sign_in_count : 1
    };

    // Getting property
    let name = user1.username;

    // Setting property, if mutable
    user1.username = String::from("wallace123");

    let user2 : User = build_user(
        String::from("thirdEmail@gmail.com"),
        String::from("username3")
    );

    let user3 = User {
        email : String::from("james@gmail.com"),
        username : String::from("james123"),
        ..user2 // This set rest of values same as user2
    };


    // Examples of Truple structs:
    struct Color(i32, i32, i32);

    struct Point(i32, i32, i32);
}

// For username and email, we dont need specificity
// Because variable name is same name as property
// This is called field init shorthand syntax
fn build_user(email : String, username : String) -> User {
    User { username, email, sign_in_count: 1, active: true }
}