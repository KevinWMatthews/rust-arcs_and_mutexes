use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let arc_copy = Arc::new(Mutex::new(0));
    let arc_boxed = Arc::new(Mutex::new(Box::new(42)));

    let cloned_copy = Arc::clone(&arc_copy);
    let cloned_boxed = Arc::clone(&arc_boxed);

    let handle = thread::spawn(move || { // Move so that references to cloned arcs are owned
        // Lock Mutex, unwrap Result, dereference MutexGuard (scoped lock)
        let value = *cloned_copy.lock().unwrap();
        println!("value: {}", value);

        // Lock Mutex, unwrap Result, dereference MutexGuard (scoped lock), dereference Box
        let value = **cloned_boxed.lock().unwrap();
        println!("value: {}", value);
    });

    handle.join().unwrap();
}
