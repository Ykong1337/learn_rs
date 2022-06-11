use std::fmt::Debug;
use std::iter::Sum;

fn main() {
    println!("Hello, world!");

    let new = NewsArticle {
        headline: "hss".to_string(),
        local: "t".to_string(),
    };

    ta(Tw{ headline: "afe".to_string(), local: "eget".to_string() });

    println!("1 : {} + {}", new.summarize1(), new.summarize1())
}

fn ta<T : Summary + Debug> (v1:T){
    println!("{}",v1.summarize1());
}

pub trait Summary {
    fn summarize1(&self) -> String {
        String::from("Read Me")
    }
}

struct NewsArticle {
    pub headline: String,
    pub local: String,
}

struct Tw {
    pub headline: String,
    pub local: String,
}

impl Summary for NewsArticle {}

impl Summary for Tw {}
