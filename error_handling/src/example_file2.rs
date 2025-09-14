use std::fs::File;


fn main(){
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}