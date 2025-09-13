use unicode_segmentation::UnicodeSegmentation;
fn main(){
    let hello = String::from("Hello");
    // let c = hello[0];

    for b in "sok".bytes(){
        println!("{}", b);
    }

    for c in "sok".chars(){
        println!("{}", c);
    }

    for g in "sok".graphemes(true){
        println!("{}", g);
    }

    // println!("c is {}", c);
}