
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        return &self.x
    }
}

// implementation specific to f32 type, i.e. not generic to all types T
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointMixed<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointMixed<X1, Y1> {
    // method definition can declare different type parameters from impl
    fn mixup<X2, Y2>(self, other: PointMixed<X2, Y2>) -> PointMixed<X1, Y2> {
        PointMixed {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p1 = PointMixed { x: 5, y: 10.4 };
    let p2 = PointMixed { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
