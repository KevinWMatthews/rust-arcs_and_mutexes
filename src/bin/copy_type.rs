// TODO review why the data inside the mutex is mutable (interior mutability)

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Mutex wraps a Copy type
    let arc = Arc::new(Mutex::new(0));

    println!("arc: {:?}", arc);

    let arc0 = Arc::clone(&arc);
    let handle0 = thread::spawn(move || { // Move the cloned arc into the thread so it can't go out of scope
        // Can't just dereference - can't move out of an Arc and Mutexes are non-Copy
        // Compiler error - cannot move out of an `Arc`
        /*
        let mutex = *arc0;
        */

        // Dereference and borrow the Mutex
        let mutex = &*arc0;

        let result = mutex.lock();

        // A MutexGuard is a scoped lock.
        // TODO is it safe to unwrap or should we match?
        let mutex_guard = result.unwrap();

        // Can dereference - the underlying data is Copyable
        let value = *mutex_guard;
        println!("value: {}", value);
    });

    let arc1 = Arc::clone(&arc);
    // Compiler error - may outlive borrowed value
    // The thread only uses a reference to the underlying data.
    // If the main thread exits first, then its variables go out of scope and
    // there is a dangling reference.
    /*
    let handle1 = thread::spawn(|| {
        *arc1.lock().unwrap() += 1;
    });
    */

    // Fix the dangling reference by using the "move" keyword
    let handle1 = thread::spawn(move || {
        println!("arc1: {:?}", arc1);
        *arc1.lock().unwrap() += 1;
        println!("arc1: {:?}", arc1);
    });

    // Compiler error - use after move
    /*
    println!("arc1: {:?}", arc1);
    */

    // Alternatively, manually move the cloned arc into the thread.
    let arc2 = Arc::clone(&arc);
    let handle2 = thread::spawn(|| {
        let arc2 = arc2;

        println!("arc2: {:?}", arc2);
        *arc2.lock().unwrap() += 2;
        println!("arc2: {:?}", arc2);
    });

    // Compiler error - use after move
    /*
    println!("arc2: {:?}", arc2);
    */

    handle0.join().unwrap();
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("arc: {:?}", arc);
}
