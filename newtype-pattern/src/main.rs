use std::fmt;

// we want to implement Display on Vec<String>
// However, since both the trait and type are
// external to our crate, we cannot. To get around this,
// we create a tuple struct that's a thin wrapper around Vec<String>.
// This new type has no runtime performance penalty

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}