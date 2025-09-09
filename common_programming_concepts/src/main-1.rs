fn main() {
// 1. variable 
/* 
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    const SUBSCRIBER_COUNT: u32 = 100_000;
*/

// 2. Integers
/*
    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)
    let f: u8  = 255;
*/

// 3. Floating-point numbers
/*
    let f: f32 = 2.3;
    let g = 3.0;
*/

// 4. addition, subtraction, multiplication, division and remainder
/*
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    // Booleans
    let t = true;
    let f = false;
    // Charater
    let c = 'z';
    let z = '%';
*/

// 5. Compound Types
/*
    // tup
    let tup: (&str, i32) = ("Let's Get Rusty!", 100_000);
    let (channel, sub_count) = tup;
    let sub_count = tup.1;

    let error_code = [200, 404, 500];
    let not_found = error_code[1];

    let byte = [0; 8];
    // println!("byte is {:#?}", byte);
*/

// 6. can funtion
    // my_function(64);

    let sum = my_function(32, 64);
    println!("The sum is: {}", sum);

}

// 6. function
//
    // fn my_function(_x: i32){
    //     println!("Another function.");
    // }

    fn my_function(x: i32, y: i32) -> i32{
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
        // let sum = x + y;
        // return sum;
        // sum
        x + y
    }
//