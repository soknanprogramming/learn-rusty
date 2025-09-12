fn main(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six is : {:#?}", six);
    println!("none is : {:#?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    /* 
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
    */
    // or we can write
    match x {
        Some(i) => Some(i + 1),
        _ => None
    }
}