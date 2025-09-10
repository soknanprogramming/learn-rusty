fn main(){
    // let mut s = String::from("hello world");
    let s2 = "hello world";

    let word = first_word(s2);
    println!("{}", word);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}