pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Box<dyn Draw> is a trait object,
    // it's stand-in for any type inside a box that implements Draw.
    // This is a different from using generic type parameters with trait bounds (e.e. Vec<Box<T>> where T: Draw or Vec<T>)
    // With generics, an instance of this vector would have elements of the same type
    // With trait objects, you can have objects of different types in the same vector, so long as they implement Draw
    // Ruse uses pointers inside the trait object to know which method to call.
    // This is conceptually similar to runtime polymorphism with v-tables or method tables
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run (&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button {}", self.label);
    }
}