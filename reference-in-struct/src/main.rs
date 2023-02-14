// This lifetime parameter 'a means that the instance of
// ImportantExcerpt cannot go out of scope before the reference it holds in part
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // We don't need to specificy lifetimes explicitly here
    // Elision rule 1: compiler annotates the sole reference param with a lifetime
    fn level(&self) -> i32 {
        3
    }

    // We don't need to explicitly specify lifetimes here
    // Elision rule: 1: compiler annotates each of the reference params with implicit lifetimes
    // Elision rule: 3: compiler applies the &self lifetime to the output reference 
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = ImportantExcerpt {
        part: first_sentence
    };
}