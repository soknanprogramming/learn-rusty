#[derive(Debug)]

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main(){
    let mut user1 = User{
        username: String::from("Soknan"),
        email: String::from("soknan@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;
    user1.username = String::from("wallace123");

    println!("user1 is {:?}", user1);

    let user2 = build_user(String::from("hello"), String::from("hello@gmail.com"));
    println!("user2 is {:?}", user2);

    let user3 = User{
        email: String::from("james@gmail.com"),
        username: String::from("james123"),
        ..user2
    };
    println!("user3 is {:?}", user3);

}

fn build_user(email: String, username: String) -> User{
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}