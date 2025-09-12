mod back_of_house{
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant(){
    let mut meal = back_of_house::Breakfast::summer("peaches");

    let meal2 = back_of_house::Breakfast{
        toast: String::from("Wheat"),
        seasonal_fruit: String::from("peaches"), // Error
    };

    meal.toast = String::from("Wheat"); 
}