struct Droppable {
    name: & 'static str
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping: {}", self.name);
    }
}

fn main() {
    let _a = Droppable {name: "Outer loop"};

    {
        let _b = Droppable {name: "first inner loop"};

        {
            let _c = Droppable{name: "second inner"};
        }
    }
}