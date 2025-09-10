fn main() {
    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");
    // let s2 = s1; // move
    let s2 = s1.clone();

    println!("s1 is: {}", s1);
    println!("s2 is: {}", s2);
}