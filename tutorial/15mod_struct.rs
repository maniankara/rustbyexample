mod struct_mod {

    pub struct WhiteBox<T> {
        pub contents: T
    }

    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T
    }


    impl<T> BlackBox<T> {
        pub fn new (contents: T) -> BlackBox<T> {
            BlackBox {
                contents: contents
            }
        }
    }
}

fn main() {
    let white_box = struct_mod::WhiteBox { contents: "Public data"};
    println!("white box: {}", white_box.contents);

    // let black_box = struct_mod::BlackBox { contents: "private data"}; // ERROR: field `contents` is private
    let black_box = struct_mod::BlackBox::new("private data"); // works as new is pub
    // println!("black box: {}", black_box.contents); // ERROR: error: field `contents` of struct `struct_mod::BlackBox` is private


}
