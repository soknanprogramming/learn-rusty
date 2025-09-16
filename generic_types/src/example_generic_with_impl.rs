struct Point<T>{
    x: T,
    y: T,
}

impl<U> Point<U> {
    fn x(&self) -> &U{
        &self.x
    }
}

impl Point<f64>{
    fn y(&self) -> f64{
        self.y
    }
}

fn main(){
    let test1 = Point{ x: 3, y: 2};
    let char = test1.x();
    println!("char is {:#?}", char);

    let test2 = Point{x: 4.0, y: 1.1};
    let y = test2.y();
}