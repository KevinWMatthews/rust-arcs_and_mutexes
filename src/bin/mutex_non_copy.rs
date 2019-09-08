use std::sync::Mutex;

fn main() {
    let boxed = Box::new(42);

    // Get the value from the Box using the Deref trait
    let value = *boxed;
    println!("value: {}", value);

    let mutex = Mutex::new(boxed);

    {
        // Lock the mutex and get a LockResult
        let lock_result = mutex.lock();

        // Unwrap the Result, getting a MutexGuard
        let mutex_guard = lock_result.unwrap();

        // Dereference the MutexGuard and *borrow* the value within.
        // Boxes can not be copied, but MutexGuards do not allow their
        // value to be moved out of a dereference!
        let boxed = &*mutex_guard;

        // Dereference the reference, then use the Deref trait to get the value from the Box
        let value = **boxed;
        println!("value: {}", value);
    }

    {
        let value = **mutex.lock().unwrap();
        println!("value: {}", value);
    }


    //

    {
        let lock_result = mutex.lock();

        // Tell the MutexGuard to allow its inner value to be changed
        let mut mutex_guard = lock_result.unwrap();

        // Borrow the Box within the MutexGuard, mutably
        let boxed = &mut *mutex_guard;

        // Change the value within the Box
        **boxed += 1;

        println!("value: {}", **boxed);
    }

    println!("mutex: {:?}", mutex);

    {
        **mutex.lock().unwrap() += 1;
        println!("mutex: {:?}", mutex);
    }
}
