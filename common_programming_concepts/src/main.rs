fn main(){
// 1. Control Flow
    /* 
    let number = 5;
    if number < 10 {
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true");
    } else{
        println!("condition was false");
    }
    */
    /* 
    let condition = true;
    let number = if condition {5} else {6};
    println!("Number is {}", number);
    */
// 2. Loop
    // loop
    /*
    let mut counter = 0;
    let result = loop{
        counter += 1;
        println!("again!");
        
        if counter == 10{
            break counter;
        }
    };
    println!("The result is {}", result);
    */
    // while
    /* 
    let mut number = 3;
    while number != 0{
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
    */

    // Control Flow
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..4{
        println!("{}!", number);
    }
}