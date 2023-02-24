
// since this is a recursive structure,
// we use indirection by boxing the "child"
// structures to the heap, this way
// the List doesn't have an infinite size
#[derive(PartialEq)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

struct ListIterator<'a>{
    cur: &'a List,
}

impl<'a> ListIterator<'a> {
    fn new(list: &List) -> ListIterator {
        ListIterator { cur: list }
    }
}

impl<'a> Iterator for ListIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur {
            Cons(val, next) => {
                self.cur = next;
                Some(val.clone())
            },
            Nil => None
        }
    }
}

impl List {
    fn iter(&self) -> ListIterator {
        ListIterator::new(self)
    }
}

fn print_list(list: &List) {
    match list {
        Cons(val, next) => {
            println!("{val}");
            print_list(next);
        }
        _ => {}
    }
}

fn main() {
    let list =
    Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    println!("Print list using loop");
    let mut cur = &list;
    while *cur != Nil {
        if let Cons(val, next) = cur {
            println!("{val}");
            cur = next;
        }
    }
    
    println!("Print list using recursion:");
    print_list(&list);

    println!("Print list using iterator:");
    for val in list.iter() {
        println!("{val}")
    }
}
