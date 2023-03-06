// Refactoring the blog-oop sample using a more idiomatic Rust approach
// Instead of creating State trait object that can be
// implemented by different states, we create
// different stages of the Post as different struct types.
// We also don't implement methods in states where they are not
// valid (e.g. content() is only available in Post struct)

pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        // start out as draft
        DraftPost {
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        // self is moved into this method
        // meaning the DraftPost instance can no
        // longer be used after transitioning to PendingReviewPost
        PendingReviewPost {
            content: self.content
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }
}
