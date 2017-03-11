pub fn public_function() {
    println!("This is a public function");
}

fn private_function() {
    println!("This is a private function");
}

pub fn indirect_call() {
    private_function();
    println!("Indirect call");
}

