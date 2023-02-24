struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

fn main() {
    let c1 = CustomSmartPointer {
        data: String::from("c1")
    };
    let c2 = CustomSmartPointer {
        data: String::from("c2")
    };
    let c3 = CustomSmartPointer {
        data: String::from("c3")
    };

    println!("CustomSmartPointer data created");

    // You can't call the trait's drop method directly, e.g. c1.drop() will not compile
    // But you can call the drop function and pass the object to drop as a parameter
    drop(c2); // this moves c2 into drop, and it will be dropped when it goes out of scope in that function
    println!("Dropped c2 before the end of the function");

    // c3 and c1 will be dropped at the end of main's scope
}
