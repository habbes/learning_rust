// How to disambiguate between
// different function implementations
// with the same name, but no receiver (self) parameter?

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

fn main() {
    // this calls baby_name implemented on Dog, not the trait implementation
    println!("A baby dog is called {}", Dog::baby_name());
    // To call the trait implementation, we use the
    // fully-qualified syntax
    // If we just called Animal::baby_name(), there could be many types
    // that implement Animal::baby_name(), and without a self param,
    // Rust can't tell which one we want to call
    println!("A baby dog is called {}", <Dog as Animal>::baby_name());
}
