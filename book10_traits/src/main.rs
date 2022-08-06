#[allow(unused)]
use std::cmp::PartialOrd;

trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    username: String,
    text: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        String::from(format!("@{}: {}", self.username, self.text))
    }
}

struct NewsLetter {
    title: String,
    location: String,
    author: String,
    text: String,
}
impl Summary for NewsLetter {
    fn summarize(&self) -> String {
        String::from(format!("{}: {}", self.author, self.text))
    }
}

fn print_summarize(news: &impl Summary) {
    println!("{}", news.summarize())
}

fn largest<T: PartialOrd>(set: &[T]) -> &T {
    let mut max = &set[0];
    for item in set {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    let tweet = Tweet {
        username: "droox".to_string(),
        text: "hello_world!".to_string(),
        reply: false,
        retweet: false,
    };
    let newslet = NewsLetter {
        title: "Wow!".to_string(),
        location: "At home".to_string(),
        author: "Daniel Roox".to_string(),
        text: "Something strange is happening...".to_string(),
    };

    print_summarize(&tweet);
    print_summarize(&newslet);

    let nums = vec![1, 2, 223, 211, 223, 4, 5, 6, 7];
    let chars = vec!['a', 'b', 'c', 'd', 'e'];

    println!("Largest in nums: {}", largest(&nums));
    println!("Largest in chars: {}", largest(&chars));

    let x = String::from("hello");
    let y = String::from("world");
    let result = longest(x.as_str(), y.as_str());
    println!("result: {}", result);
}

// lifetime
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}
