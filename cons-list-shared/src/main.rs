// We want to be able to share a list between
// other list. In that case, the sub list will have
// multiple owners. We use the reference-counting
// smart pointer to achieve that.
// Rc allows you to share immutable references
// Note: Rc should not be used across multiple threads (probably because the ref counting is not thread safe)

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let mut a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // strong_count function use to get the current (strong) ref count
    println!("count after creating a = {}", Rc::strong_count(&a));
    // Rc::clone does not clone the data, it clones the pointer to the data and increments the ref count.
    // So essentially we have multiple pointers to the same data
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
