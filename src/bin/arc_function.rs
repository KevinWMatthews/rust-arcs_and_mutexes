use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let arc = Arc::new(Mutex::new(0));

    let arc1 = Arc::clone(&arc);
    let thread1 = spawns_thread(arc1);
    // println!("arc1: {:?}", arc1);        // Error: arc1 has been moved!

    let arc2 = Arc::clone(&arc);
    let thread2 = spawns_thread(arc2);

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("arc: {:?}", arc);
}

fn spawns_thread(arc: Arc<Mutex<i32>>) -> JoinHandle<()> {
    let handle = thread::spawn(move || {
        println!("arc: {:?}", arc);
        *arc.lock().unwrap() += 1;
    });
    handle
}
