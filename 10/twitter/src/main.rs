pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("({}さんの文章をもっと読む...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("緊急ニュース!! {}", item.summarize());
}

fn returns_tweet_summarizable() -> impl Summary {
    Tweet {
        username: String::from("jdkfx"),
        content: String::from(
            "こんにちは! 私の名前はハルキです!",
        ),
        reply: false,
        retweet: false,
    }
}

fn returns_article_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("本格的な夏がやってきた!"),
        location: String::from("広島県, 日本"),
        author: String::from("jdkfx"),
        content: String::from(
            "猛暑日が続き、熱中症などの対策が必要になってきます!\
            不要不急の外出は控えましょう!",
        ),
    }
}

fn main() {
    let tweet = returns_tweet_summarizable();
    let article = returns_article_summarizable();

    println!("1件の新しいツイート: {}", tweet.summarize());
    println!("新しい記事があります! {}", article.summarize());
}