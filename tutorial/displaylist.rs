use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let List(ref vec) = *self;

        try!(write!(f, "["));

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { try!(write!(f, ", "))}
            // try!(write!(f, "{}", v))
            try!(write!(f, "{c}:{v:2}", c=count,v=v)) // activity
        }

        write!(f, "]")
    }
}


fn main() {
    let v = List(vec![1,2,3]);

    println!("{}", v);
}