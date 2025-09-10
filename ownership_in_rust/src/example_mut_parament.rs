mod ts;

fn main(){
    let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("The length of '{}' is {}.", s2, len);

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("s1 = {}", s1);
}
/*
fn calculate_length(s : String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
 */
fn calculate_length(s: &String) -> usize {
    // s.push_str("oops"); // Error: it not mut
    let length = s.len();
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}