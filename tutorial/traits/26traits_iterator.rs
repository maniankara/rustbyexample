struct Fibonacci {
    curr: u32,
    next: u32
}

impl Iterator for Fibonacci {

    type Item = u32; // missing type in implementation ????%/&)?=

    fn next(&mut self) -> Option<u32> {

        let _new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = _new_next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1}
}

fn main() {
    for i in fibonacci().take(4) {
        println!("{}", i);
    }

    for i in fibonacci().skip(4).take(4) {
        println!("{}", i);
    }
}