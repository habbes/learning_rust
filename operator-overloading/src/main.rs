use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Add is a generic trait with a default
// type parameter Add<Rhs=Self>
// Implementing Add adds + operator support
// for custom types.
// The default generic type param makes sense because most of the
// time you'd want to add a type to itself.
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        // using pattern matching/destructuring
        // let Millimeters(millimeters,) = self;
        // let Meters(meters) = other;
        // Millimeters(millimeters + 1000 * meters)

        Millimeters(self.0 + 1000 * other.0)
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3},
        Point { x: 3, y: 3 }
    );

    assert_eq!(
        Millimeters(100) + Meters(2),
        Millimeters(2100)
    );
}
