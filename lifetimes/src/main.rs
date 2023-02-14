fn main() {
    let string1 = String::from("abcd");
    let result;
    //{ // creating a new scope here will cause compilation to fail because string2 might not live long enough
    let string2 = String::from("xyz");

    result = longest(string1.as_str(), string2.as_str());
    //}
    // Note string literals use the special 'static lifetime, which means they're always alive during the lifetime of the process
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}