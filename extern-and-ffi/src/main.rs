// this extern block lists the signatures
// of external functions from another language we want to call
// The "C" part defines the ABI the external function uses,
// In this case we specify the C calling conventions
extern "C" {
    fn abs(input: i32) -> i32;
}

// Note, we can also define functions that other languages can
// call using the extern keyword
// The #[no_mangle] attribute tells Rust compiler not to mangle the name
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn main() {
    // it's "unsafe" to call a function defined in another language
    // since Rust can't check that code
    unsafe {
        println!("Abs of -3 according to C: {}", abs(-3));
    }
}
