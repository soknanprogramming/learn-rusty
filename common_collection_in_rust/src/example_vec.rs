fn main() {
    let a = [1, 2, 3];
    let mut v:Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3];

    let third = &v[2];
    v2.push(6);
    // let third = &v[20]; // error
    println!("The third element is {}", third);

    match v.get(20){
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
