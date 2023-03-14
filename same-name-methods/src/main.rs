trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    // method implemented on the type directly, outside of any traits
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}


fn main() {
    let person = Human;

    // by default, the method implemented on the type itself is called
    person.fly(); // could also be written as Human::fly(&person)
    // For trait-specific methods, the trait is explicitly specified
    Wizard::fly(&person);
    Pilot::fly(&person);
}
