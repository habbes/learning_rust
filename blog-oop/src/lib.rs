pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        // To consume the old state, the request_review method needs to take ownership of the state value.
        // This is where the Option in the state field of Post comes in:
        // we call the take method to take the Some value out of the state field and leave a None in its place,
        // because Rust doesn’t let us have unpopulated fields in structs.
        // This lets us move the state value out of Post rather than borrowing it.
        // Then we’ll set the post’s state value to the result of this operation.
        // We need to set state to None temporarily rather than setting it directly with code like
        // self.state = self.state.request_review(); to get ownership of the state value.
        // This ensures Post can’t use the old state value after we’ve transformed it into a new state.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn content(&self) -> &str {
        // We call the as_ref method on the Option because we want a reference to the value inside the Option rather than ownership of the value.
        // Because state is an Option<Box<dyn State>>, when we call as_ref, an Option<&Box<dyn State>> is returned.
        // If we didn’t call as_ref, we would get an error because we can’t move state out of the borrowed &self of the function parameter.

        // We then call the unwrap method, which we know will never panic,
        // because we know the methods on Post ensure that state will always contain a Some value when those methods are done.
        // We know that a None value is never possible,
        // even though the compiler isn’t able to understand that.
        self.state.as_ref().unwrap().content(self)
    }
}

trait State {
    // This method takes Box<Self> meaning that
    // the method is only valid when called on a Box holding the type.
    // This syntax takes ownership of Box<self>, invalidating
    // the old state so that the state value of the Post can
    // transform into a new state
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}