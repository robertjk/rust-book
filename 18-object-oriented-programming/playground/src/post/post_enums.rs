pub struct Post {
    state: Option<State>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(State::Draft),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content
            .push_str(self.state.as_ref().unwrap().add_text(text));
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.reject());
        }
    }
}

enum State {
    Draft,
    PendigReview(usize),
    Published,
}

impl State {
    fn add_text<'a>(&self, added_text: &'a str) -> &'a str {
        match self {
            State::Draft => added_text,
            _ => "",
        }
    }

    fn request_review(self) -> Self {
        match self {
            State::Draft => State::PendigReview(0),
            _ => self,
        }
    }

    fn approve(self) -> Self {
        match self {
            State::PendigReview(0) => State::PendigReview(1),
            State::PendigReview(_) => State::Published,
            _ => self,
        }
    }

    fn reject(self: Self) -> Self {
        match self {
            State::PendigReview(_) => State::Draft,
            _ => self,
        }
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        match self {
            State::Published => &post.content,
            _ => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Post;

    #[test]
    fn positive_post_lifecycle() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    #[test]
    fn post_lifecycle_with_reject() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.reject();
        assert_eq!("", post.content());

        post.approve();
        post.approve();
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    #[test]
    fn adding_text_works_only_in_draft() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        post.add_text("Something");
        assert_eq!("", post.content());

        post.approve();
        post.add_text("Something");
        assert_eq!("", post.content());

        post.approve();
        post.add_text("Something");
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
