fn main(){
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // can not mut 1 variable more that one time
    // println!("r1 = {}, r2 = {}", r1, r2);

    let mut s2 = String::from("hello");
    let r1 = &s2;
    let r2 = &s2;
    // let r3 = &mut s2; // cannot borrow
    println!("r1 = {}, r2 = {}", r1, r2);
    let r3 = &mut s2;
    println!("r3 = {}", r3);

}