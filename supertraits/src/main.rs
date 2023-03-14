use std::fmt;

// OutlinePrint can only be implemented
// on types that implement Display
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // the to_string() method is guaranteed by the Display trait
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn main() {
    let p = Point { x: 10, y: 20, };
    println!("{:?}", p.outline_print());
}
