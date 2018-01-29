struct Post {
    state: Option<Box<PostState>>,
    content: String,
}

impl Post {
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    
    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    fn new() -> Post {
        Post { state: Some(Box::new(NewState{})), content: String::from("") }
    }
}

trait PostState {
    fn request_review(self: Box<Self>) -> Box<PostState>;
    fn approve(self: Box<Self>) -> Box<PostState>;
    fn content <'a> (&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct NewState {
}

impl PostState for NewState {
    fn request_review(self: Box<Self>) -> Box<PostState> {
        Box::new(UnderReview{})
    }

    fn approve(self: Box<Self>) -> Box<PostState> {
        self
    }
}

struct UnderReview {
}

impl PostState for UnderReview {
    fn request_review(self: Box<Self>) -> Box<PostState> {
        self
    }

    fn approve(self: Box<Self>) -> Box<PostState> {
        Box::new(Approved{})
    }
}

struct Approved {
}

impl PostState for Approved {
    fn request_review(self: Box<Self>) -> Box<PostState> {
        self
    }

    fn approve(self: Box<Self>) -> Box<PostState> {
        self
    }

    fn content <'a> (&self, post: &'a Post) -> &'a str {
        &post.content[..]
    }
}
    
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

}
