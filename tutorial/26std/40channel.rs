use std::thread::spawn;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

static NTHREADS: i32 = 3;

fn main() {

    let (tmpx, rx) : (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREADS {

        let tx = tmpx.clone();

        spawn(move || {
            tx.send(id).unwrap();
            println!("Thread id: {}", id);

        });

    }

    // waiter
    for _ in 0..NTHREADS {
        println!("Ids: {:?}", rx.recv());
    }



}