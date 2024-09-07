fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//1

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
//2
pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String
}

impl Summery for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summery for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summery {
    fn summarize(&self) -> String;
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    //1
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    
    //2
    let tweet = Tweet {
        username: String::from("VJ"),
        content: String::from("Sky is falling"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        author: String::from("@Jhon Doe"),
        headline: String::from("The sky is falling"),
        content: String::from("The sky is actually not falling ....")
    };
    
    println!("Tweet Summery: {}", tweet.summarize());
    println!("Article Summery: {}", article.summarize());

}
