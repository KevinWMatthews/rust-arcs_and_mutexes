use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

fn main() {
    let arc = Arc::new(Mutex::new(0));

    let arc1 = Arc::clone(&arc);
    let thread1 = spawn_thread(arc1);
    // Compiler error: arc1 has been moved!
    /*
    println!("arc1: {:?}", arc1);
    */

    let arc2 = Arc::clone(&arc);
    let thread2 = spawn_thread(arc2);

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("arc: {:?}", arc);
}

fn spawn_thread(arc: Arc<Mutex<i32>>) -> JoinHandle<()> {
    let handle = thread::spawn(move || {
        println!("arc: {:?}", arc);
        *arc.lock().unwrap() += 1;
    });
    handle
}
