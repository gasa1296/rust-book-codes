#![allow(unused)]
pub trait Summary {
    //Default Implementations
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
//Implementing a Trait on a Type
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//Traits as Parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//Trait Bound Syntax
//pub fn notify(item1: &impl Summary, item2: &impl Summary)
//pub fn notify<T: Summary>(item1: &T, item2: &T)
pub fn notify0<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

//Specifying Multiple Trait Bounds with the + Syntax
//pub fn notify(item: &(impl Summary + Display))
//pub fn notify<T: Summary + Display>(item: &T)

//Clearer Trait Bounds with where Clauses
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32
/*
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
*/

//Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

//Using Trait Bounds to Conditionally Implement Methods
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}