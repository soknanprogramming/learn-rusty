fn main(){
    // String are stored as a collection of UTF-8 encoded bytes
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    let mut s = String::from("bar");
    s.push_str("bar");
    s.push('!');

    println!("s is : {}", s);
}
