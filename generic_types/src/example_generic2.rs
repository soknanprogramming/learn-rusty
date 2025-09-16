struct Point<T, U> {
    x: T,
    y: U
}

fn main(){
    let p = Point {x: 14, y: 5.0};
    let p2 = Point {y: String::from("me"), x: String::from("you")};
}