use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let arc = Arc::new(Mutex::new(Box::new(0)));

    let arc1 = Arc::clone(&arc);

    let handle1 = thread::spawn(move || {
        println!("arc1: {:?}", arc1);

        // Compiler error - cannot move out of an `Arc`
        // This isn't because the Box is non-Copy;
        // it's because the Mutex can't is non-Copy.
        // let mutex = *arc1;

        // Must borrow
        let mutex = &*arc1;

        let result = mutex.lock();

        let mutex_guard = result.unwrap();

        // Can't dereference the MutexGuard because Boxes are non-Copy and
        // we can't move from behind a borrow
        // Compiler error: cannot move out of dereference...
        /*
        let boxed = *mutex_guard;
        */
        let boxed = &*mutex_guard;

        // Can't dereference once - this simply gives us the Box!
        // Compiler error - cannot move out of `*boxed` which is behind a shared reference
        /*
        let value = *boxed;
        */

        let value = **boxed;
        println!("value: {}", value);
    });

    // Shorthand
    let arc2 = Arc::clone(&arc);
    let handle2 = thread::spawn(move || { // As with Copy types, move so that the thread owns references
        let value = **arc2.lock().unwrap();
        println!("value: {}", value);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
