use std::fs::File;
use std::io;
use std::io::Read;

fn main(){

}

fn read_username_from_files() -> Result<String, io::Error> {
    let mut s = String::new();
    /* 
    let mut f = File::open("hello.txt")?;

    f.read_to_string(&mut s)?;
    Ok(s)
    */
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

