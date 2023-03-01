use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // The closure of the spawned thread captures the variable v
    // By default, this capture would be an immutable borrow
    // because the closure only needs an immutable ref to print v.
    // But that would not compile since there's no guarantee
    // that v would continue to be valid for the entire lifetime
    // of the spawned thread. It's possible that it could be
    // dropped in the main thread while the spawned thread still
    // refers to it.
    // To fix that, we move the value into the closure.
    let handle = thread::spawn(move|| {
        println!("Print vector from spawned thread: {:?}", v);
    });

    handle.join().unwrap();
}
