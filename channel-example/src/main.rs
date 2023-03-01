use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    // channel will be closed when either tx or rx is dropped

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    // recv() is a blocking call. There's a non-blocking variant called try_recv()
    let received = rx.recv().unwrap();
    // The received value is moved here
    println!("Received value {}", received);
}
