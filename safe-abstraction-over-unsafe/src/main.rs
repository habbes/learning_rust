use std::slice;

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // we can call the function without being in unsafe block
    // this is effectively a safe abstraction over unsafe code
    // However, a faulty implementation of split_at_mut could
    // lead to buffer overflows or undefined behavior
    // so it really safe?
    // I find it a misleading that Rust allows to call a function
    // that contains an unsafe code in "safe" code. That means
    // any third-party function I call could potentially be unsafe
    // and I wouldn't know, and I'd have to trust the the function
    // author got it right
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("slice a {:?}", a);
    println!("slice b {:?}", b);
}

// splits a mutable slice into two mutable (sub) slices at a n index
// similar the built-in-method
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    // gets mutable pointer to input slice
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        // this would not be possible to achieve in "safe" rust because
        // we can't take 2 mutable references to the same slice, even if
        // the references are for disjoint sub-slices of the slice.
        // e.g. (&mut values[..mid], &mut values[mid..]) would not compile
        // Rust compiler doesn't know that the slices don't overlap
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )

        // slice::from_raw_parts_mut is an unsafe function that takes raw pointers
        // and assume it's okay to use.
        // be we can return a slice to invalid memory or cause undefined behavior
        // by passing invalid pointers, e.g.
        // slice::from_raw_parts(ptr.add(mid + 1), len - mid) would be overflow the buffer
    }
}