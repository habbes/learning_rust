
// This annotations makes the macro available
// whenever this crate is brought into scope
#[macro_export] 
macro_rules! vec_simple {
    // macro_rules! syntax is similar to the match syntax,
    // each arm has a pattern and code that should be generated if that arm is matched
    // This example has only one arm
    // We enclose the entire pattern in parentheses (I found that using square brackets and curly braces also works!)
    // We use $ to declare a variable in the macro system that will contain the Rust code matching the pattern
    // Then inside the $() we capture values to be used for replacement in code generation
    // $x:expr means match any expression and bind it to the name $x
    // The comma after $() means a literal comma separate could optionally appear after the matching code
    // The * means the preceding code can be repeated 0 or more times
    ( $($x:expr),* ) => {
        {
            let mut temp_vec = Vec::new();
            // The $()* indicates that the code inside should be repeated
            // as many times as the pattern matches
            $(
                // We use the $x expression as the argument
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

fn main() {
    let list = vec_simple![1, 2, 3];
    // I realized you can use [], () or {} interchangeably when calling macros, e.g.:
    // vec!(1, 2, 3), vec!{1, 2, 3}, println!{"Hello, World"}
    println!("{:?}", list);
}