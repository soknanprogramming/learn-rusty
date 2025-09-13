fn main(){
    /* 
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    
    // println!("s1 is : {}", s1); // error
    println!("s2 is : {}", s2);
    println!("s3 is : {}", s3);
    */

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);

    println!("s1 is : {}", s1);
    println!("s2 is : {}", s2);
    println!("s3 is : {}", s3);
}