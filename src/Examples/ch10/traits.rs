pub struct NewsArticle {
    pub author : String,
    pub headline : String,
    pub content : String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username : String,
    pub content : String,
    pub reply : bool,
    pub retweet : bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    // Can just define the name, arguments and out of the function
    // without implementing it
    // fn summarize(&self) -> String;

    // Can define a default function
    // And the function may be overwrriten
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

fn main() {
    let tweet = Tweet {
        username : String::from("@usernameExample"),
        content : String::from("Hello World"),
        reply : false,
        retweet : false
    };

    let article = NewsArticle {
        author : String::from("Exam Ple"),
        headline : String::from("The sky is Falling!"),
        content : String::from("The sky is not actually falling.")
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);

    println!("{}", returns_summarizable().summarize())
}

// &impl Summary means any type that implements Summary 
pub fn notify(item : &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Another way to implement notifiy is the following:
// pub fn notify<T: Summary>(item : &T) {
//     println!("Breaking news! {}", item.summarize());
// }
// Here T is a given type, which implements Summary

// A third way to implement notifiy: 
// pub fn notify<T>(item : &T)
//     where T: Summary
// {
//     println!("Breaking news! {}", item.summarize());
// }


// Function returns a type which implements Summary
// The function must return a single type.
// Can't return a tweet on certain input and article on another 
fn returns_summarizable() -> impl Summary {
    Tweet {
        username : String::from("horse_ebooks"),
        content : String::from("of course, as you probably already know, people"),
        reply : false,
        retweet : false
    }
}


use std::fmt::Display;
struct Pair<T> {
    x : T,
    y : T
}

impl<T> Pair<T> {
    fn new(x : T, y : T) -> Self {
        Self {x, y}
    }
}

// This is only available, if T is of type which implements Display and PartialOrd
impl<T : Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is x = {}", self.y);
        }
    }
}