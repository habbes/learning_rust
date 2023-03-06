// Refactoring the blog-oop sample using a more idiomatic Rust approach

use blog_rusty::Post;

fn main() {
    let mut post = Post::new();
    
    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    
    let post = post.approve();

    assert_eq!("I ate salad for lunch today", post.content());
}
