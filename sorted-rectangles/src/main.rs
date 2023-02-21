#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// A closure can implement one or more of the following traits:
/*
- FnOnce applies to closures that can be called once. All closures implement at least this trait, because all closures can be called.
A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
- FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values.
These closures can be called more than once.
- Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values,
as well as closures that capture nothing from their environment.
These closures can be called more than once without mutating their environment,
which is important in cases such as calling a closure multiple times concurrently.
 */

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // sort_by_key takes a closure that implements FnMut because it calls the closure multiple times
    // This also means that the closure should not move values of the environment
    // But it can mutate captured values

    let mut num_sorted_operations = 0;
    list.sort_by_key(|r| {
        num_sorted_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sorted_operations} operations", list);
}
