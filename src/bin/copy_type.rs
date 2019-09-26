use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Mutex wraps a Copy type
    let arc = Arc::new(Mutex::new(0));

    println!("arc: {:?}", arc);

    let arc1 = Arc::clone(&arc);
    let handle1 = thread::spawn(move || { // Move the cloned arc into the thread so it can't go out of scope
        // Can't just dereference - can't move out of an Arc and Mutexes are non-Copy
        // Compiler error - cannot move out of an `Arc`
        /*
        let mutex = *arc1;
        */

        // Dereference and borrow the Mutex
        let mutex = &*arc1;

        let result = mutex.lock();

        // A MutexGuard is a scoped lock.
        // TODO is it safe to unwrap or should we match?
        let mutex_guard = result.unwrap();

        // Can dereference - the underlying data is Copyable
        let value = *mutex_guard;
        println!("value: {}", value);
    });

    let arc2 = Arc::clone(&arc);
    // Compiler error - may outlive borrowed value
    // The thread only uses a reference to the underlying data.
    // If the main thread exits first, then its variables go out of scope and
    // there is a dangling reference.
    /*
    let handle2 = thread::spawn(|| {
        *arc2.lock().unwrap() += 1;
    });
    */

    // Fix the dangling reference by using the "move" keyword
    let handle2 = thread::spawn(move || {
        println!("arc2: {:?}", arc2);
        *arc2.lock().unwrap() += 1;
        println!("arc2: {:?}", arc2);
    });

    // Compiler error - use after move
    /*
    println!("arc2: {:?}", arc2);
    */

    // Alternatively, manually move the cloned arc into the thread.
    let arc3 = Arc::clone(&arc);
    let handle3 = thread::spawn(|| {
        let arc3 = arc3;

        println!("arc3: {:?}", arc3);
        *arc3.lock().unwrap() += 2;
        println!("arc3: {:?}", arc3);
    });

    // Compiler error - use after move
    /*
    println!("arc3: {:?}", arc3);
    */

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();

    println!("arc: {:?}", arc);
}
