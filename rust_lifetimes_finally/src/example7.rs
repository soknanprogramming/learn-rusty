fn main() {
    let a;
    {
        let s = "I have a static lifetime";
        a = s;
    }

    println!("a is {}", a);
    
}