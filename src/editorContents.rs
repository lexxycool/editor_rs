pub struct EditorContents {
    content: String,
} 

impl EditorContents {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn push(&mut self, ch: char) {
        self.content.push(ch);
    }

    fn push_str(&mut self, string: &str) {
        self.content.push_str(string);
    }

}