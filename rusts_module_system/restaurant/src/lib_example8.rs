/* 
use rand::Rng;
use rand::CryptoRng;
use rand::ErrorKind::Transient;
*/
// Or 
use rand::{Rng, CryptoRng, ErrorKind::Transient};
// use std::io::{self, Write};
use std::io::*;


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
    let secret_number = rand::thread_rng().gen_range(1, 101);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}