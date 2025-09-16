use std::fmt::{Debug, Display};


pub struct NewArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// pub fn notify(item: &impl Summary){
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T:Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/* 
pub fn notify(item1: &impl Summary, item2: &impl Summary){
    // ...
}
*/
/* 
pub fn notify<T: Summary>(item: &T, item2: &T) {
    // ...
}
*/
/* 
pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary){
    // ...
}

pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {
    // ...
}
*/
/* 
fn some_function<T: Display + Clone, U: Clone + Display>(t: &T, u: &U) -> i32 {
    // ...
}

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    // ...
}
*/

fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false
    };

    let article = NewArticle {
        author: String::from("John Doe"),
        content: String::from("The Sky is Falling"),
        headline: String::from("The sky is not actually falling")
    };

    notify(&article);

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}