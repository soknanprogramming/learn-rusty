fn main(){
/*
    let mut s = String::from("hello");
    let word = first_word(&s);
    s.clear()
*/
    let mut s = String::from("hello world");
    // let hello = &s[0..5]; // we can also write
    let hello = &s[..5];
    // let world = &s[6..11]; // we can also write
    let world = &s[6..];

    let world = first_word(&s);
    s.clear();

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}