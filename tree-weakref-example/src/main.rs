use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: Vec<Rc<Node>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: vec![]
    });

    // We attempt to obtain a strong reference to the parent
    // If has already been dropped or not yet allocated, None is returned
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // 1 strong ref to leaf, 0 weak refs
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: vec![Rc::clone(&leaf)]
        });

        // update the parent of the leaf
        // we borrow a mutable ref of the parent RefCell
        // And assign it weak reference to the parent (via Rc::downgrade)
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // 1 strong ref to branch from the branch = Rc::new variable
        // 1 weak ref to branch from leaf.parent
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        // 2 strong refs to leaf: 1 from the leaf var, the other from
        // the child in branch
        // 0 weak refs
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        // note that weak refs are printed as (Weak) instead of being expanded
        // this avoid infinite print and stack overflow
        println!("leaf parent in inner scope = {:?}", leaf.parent.borrow().upgrade());
    }

    println!("leaf parent in outer scope = {:?}", leaf.parent.borrow().upgrade());
    // 1 strong ref (parent branch as gone out of scope and got dropped)
    // 0 weak refs
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
