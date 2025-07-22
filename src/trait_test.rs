use std::fmt::format;
use std::fmt::Display;

pub fn main(){
    test_largest();
}

fn largest<T:PartialOrd+Copy>(list: &[T]) -> T {
    //为什么需要partialord和copy trait？
    // PartialOrd: 用于比较大小，Copy: 用于复制值。
    // 为什么需要Copy？在i32和char这样类型是已知大小的时候可以储存在栈上，他们都实现了Copy trait。
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn test_largest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

pub trait Summary{
    fn summarize(&self) -> String{
        String::from("default summary")
    }
}


pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//在 Rust 中，你不可以用 + 这种方式在一个 impl 块中同时实现多个 trait，
// 例如 impl Summary + huashen for NewArticle 是错误的语法。
impl Summary for NewArticle{
    fn summarize(&self) -> String {
       format!("{}:{}",self.headline, self.location)
    }
}


fn return_summarizable() -> impl Summary {
    NewArticle {
        headline: String::from("Breaking News"),
        location: String::from("New York"),
        author: String::from("John Doe"),
        content: String::from("This is the content of the article."),
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

//只为实现Display和PartialOrd的类型实现cmp_display方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

