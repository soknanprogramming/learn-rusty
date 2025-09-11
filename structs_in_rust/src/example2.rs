#[derive(Debug)]
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Color(i32, i32, i32);
fn main(){
    
    struct Point(i32, i32, i32);

    let color = Color(1, 2, 3);
    let Point(x, y, z) = Point(1, 2, 3);
    println!("color is : {:#?}", color);
    println!("x is {}, y is {}, z is {}", x, y, z);
}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}