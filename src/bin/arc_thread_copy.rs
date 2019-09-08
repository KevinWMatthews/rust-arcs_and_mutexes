use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let arc = Arc::new(Mutex::new(0));

    // Could clone only once if we don't need to access the shared data again
    let arc1 = Arc::clone(&arc);
    let arc2 = Arc::clone(&arc);

    let handle1 = thread::spawn(move || {
        println!("arc1: {:?}", arc1);
        *arc1.lock().unwrap() += 1;
    });
    // println!("arc1: {:?}", arc1);

    // The `move` keyword forces references to be moved into the closure.
    // If `move` is not specified, the borrow checker fails:
    // the closure only uses references and this thread may outlive the main thread,
    // which causes a dangling reference.
    // We can manually move the arc into the thread.
    let handle2 = thread::spawn(|| {
        // This is why `move` is used
        let arc2 = arc2;

        println!("arc2: {:?}", arc2);
        *arc2.lock().unwrap() += 2;
    });
    // println!("arc2: {:?}", arc2);

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("arc: {:?}", arc);
}
