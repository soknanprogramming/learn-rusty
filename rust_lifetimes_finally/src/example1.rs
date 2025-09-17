fn main() {
    let r;
    {
        let x =5;
        r = &x; // x is drop when scape end
    }
    println!("r: {}", r);
}


fn main2{
    let x = 5;
    let r = &x;
    println!("r: {}", r);
}