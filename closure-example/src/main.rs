use std::thread;

fn main() {
    // immutable_borrow();
    // mutable_borrow();
    take_ownership();
}

fn immutable_borrow() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn mutable_borrow() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // since the closure mutably borrows the list
    // we also have to declare the binding as mutable, otherwise it won't compile
    let mut borrows_mutably = || list.push(7);

    // we can't borrow an immutable reference to the list while the closure
    // currently holds a mutable reference to the list
    // println!("Before calling closure: {:?}", list);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn take_ownership() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // if the closure takes an immutable reference to list
    // and runs on a different thread from where list is defined
    // it's possible list can go out of scope before the thread that
    // runs the closure complete.
    // For this reason, we force the closure to take ownership of the list
    // (and any other object it would reference) using the move keyword
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}