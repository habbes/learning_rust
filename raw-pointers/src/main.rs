fn main() {
    let mut num = 5;

    // raw pointers are not subject to Rust's ownership rules:
    // You can hold mutable and immutable raw pointers to the
    // same data at the same time
    // We can create raw pointers in 'safe' code
    // *const i32 is an immutable pointer to an i32
    let r1 = &num as *const i32;
    // *mut i32 is a mutable pointer to an i32
    let r2 = &mut num as *mut i32;

    unsafe {
        // we can mutate the data through the mutable pointer
        *r2 = *r1 + 1;
        // to dereference pointers and read the data
        // we need to be in unsafe code
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);

        // raw pointers aren't guaranteed to point to valid memory
        // Are allowed to be null
        // don't implement automatic cleanup
    }
}
