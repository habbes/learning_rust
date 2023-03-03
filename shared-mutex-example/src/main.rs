use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // Arc performs atomic reference counting
    // which makes it suitable for sharing references
    // across multiple threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Trying to move an Rc to a different thread will fail
        // because it does not implement the Send trait
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Mutex.lock() blocks until this thread acquires the lock
            // It returns a MutexGuard smart pointer which releases the lock when it goes
            // out of scope.
            let mut val = counter.lock().unwrap();
            // The counter mutex is immutable, but its interior mutability allows
            // us to modify the value inside
            *val = *val + 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let total = counter.lock().unwrap();
    println!("Total is {}", total);
}