fn main(){
    let x = 5;
    // let y = Some(5);
    let y = None;

    // let sum = x + y; // error
    let sum = x + y.unwrap_or(0);

    println!("sum is : {}", sum);
}