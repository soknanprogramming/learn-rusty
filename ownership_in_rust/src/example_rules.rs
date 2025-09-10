fn main() {
    // -- Owership rules ---
    // 1. Each value in Rust has a variable that's called it owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    { // s is not valid here, it's not yet declared
        let s = "hello";
        // do stuff with s
        println!("s is : {}", s);
    }// this scope is now over, and s is no longer valid
}

/*
The Rules of References
1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.
 */
