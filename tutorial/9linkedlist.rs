use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil
}

impl List {

    fn new() -> List {
        Nil
    }

    fn prepend(self, val: u32) -> List {
        Cons(val, Box::new(self))
    }

    fn len(&self) -> i32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => format!("Nil")
        }
    }
}


fn main() {
    let mut list = List::new();

    list = list.prepend(4);
    list = list.prepend(3);
    list = list.prepend(2);
    list = list.prepend(1);

    println!("{}", list.stringify());

    println!("Length: {}", list.len());
}