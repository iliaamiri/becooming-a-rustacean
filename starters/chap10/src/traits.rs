pub fn demo() {
    let tweet = Tweet {
        username: "something".to_string(),
        content: "content".to_string(),
        reply: false,
        retweet: false,
    };

    tweet.test();
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

trait Summary {
    fn summarize(&self) -> String;

    fn test(&self) -> () {
        println!("hello");
    }
}

trait Author {
    fn author(self) -> String;
    fn authored_at(self) -> u64;
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        let mut action = "tweeted";
        if self.reply {
            action = "replied";
        }
        if self.retweet {
            action = "retweeted";
        }

        format!("{} {} something", self.username, action)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        "this is my news summary".to_string()
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking!!! {}", item.summarize());
}

fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Breaking!!! {}", item.summarize());
}
