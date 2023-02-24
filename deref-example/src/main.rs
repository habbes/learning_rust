use std::ops::Deref;

struct MyBox<T>(T, i32);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x, 0)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // deref should return a reference to the inner data
        &self.0
    }
}

// there's also a DerefMut trait for mutable references

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // behind the scenes, Rust will call *(y.deref())

    let name = MyBox::new(String::from("Rust"));
    // Deref coercion converts a reference of type that implements Deref
    // to a reference of another type. In this case it allows us
    // to pass a reference to MyBox<String> to a function that expects
    // a &str argument.
    // Behind the scene, Rust can call name.deref() to get a &String
    // then call String.deref() to get a &str.
    // Rust will call the deref() function as many times as necessary
    // to convert the initial type to the target type.
    // This is resolve at compile time with no runtime penalty.
    hello(&name);
    // Without deref coercion we would call &(*name)[..]
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}