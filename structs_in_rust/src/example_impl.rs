#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self,  other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle{
    fn square(size: u32) -> Rectangle{
        Rectangle { width: size, height: size }
    }
}

fn main(){
    let rect = Rectangle{
        width: 10,
        height: 50
    };

    println!("rect: {:?}", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect1 = Rectangle{
        width: 5,
        height: 5
    };

    let rect2 = Rectangle{
        width: 9,
        height: 50
    };

    if rect.can_hold(&rect1){
        println!("The rectangle is already holding.");
    } else {
        println!("It can't hold.");
    }

    if rect.can_hold(&rect2) {
        println!("The rectangle is already hold.");
    } else {
        println!("It doesn't hold.");
    }


    let new_rect = Rectangle::square(3);
    println!("new_rect is : {:?}", new_rect);
}