fn main() {
    let array: [i32; 4] = [1, 2, 3, 4];
    println!("array is : {:?}", array);


    let number_slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("number_slice is : {:?}", number_slice);

    let animal_slice: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("animal_slice is : {:?}", animal_slice);

    let book_slices: &[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("book_slices is : {:?}", book_slices);

    // String Vs String Slices (&str)
    // String [growable, mutable, owned string type]

    let mut stone_cold: String = String::from("Helle, ");
    stone_cold.push_str("Yeah !");
    println!("Stone Cold Says: {}", stone_cold);

    // B- &str (String Slice)
    let string: String = String::from("Hello, World!");
    // let slice: &str = &string;
    let slice: &str = &string[0..5];
    println!("Slice Value : {}", slice);
}

// fn print(){
//     println!("SLICE: {}", slice);
// }
