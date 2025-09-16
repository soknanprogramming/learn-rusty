use std::fmt::Display;

struct Pair<T> {
    x: T, 
    y: T,
}

impl<T> Pair<T>{
    fn new_value(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x >= self.y {
            print!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main(){
    let p = Pair::new_value(3, 8);
    p.cmp_display();

}