use crate::traits::Summary;

pub struct Tweets {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweets {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
