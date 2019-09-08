use std::sync::{Arc, Mutex};
use std::sync::{LockResult, MutexGuard};

fn main() {
    let arc = Arc::new(Mutex::new(0));
    println!("arc: {:?}", arc);

    {
        // Dereferencing the Arc gives the Mutex, which can not be moved
        let mutex: &Mutex<i32> = &*arc;

        // Lock the mutex, which apparenty can be done on a &Mutex?
        // Yes - methods apply the Deref trait.
        let lock_result: LockResult<MutexGuard<i32>> = mutex.lock();

        let mutex_guard: MutexGuard<i32> = lock_result.unwrap();

        let value: i32 = *mutex_guard;

        println!("value: {}", value);
    }

    {
        // Careful - string formatting applies the Deref trait!
        let value: i32 = *arc.lock().unwrap();
        println!("value: {}", value);
    }
    println!("arc: {:?}", arc);

    {
        let mutex = &*arc;

        let lock_result = mutex.lock();

        let mut mutex_guard = lock_result.unwrap();

        *mutex_guard += 1;

        let value = *mutex_guard;
        println!("value: {}", value);
        println!("arc: {:?}", arc);
    }

    {
        *arc.lock().unwrap() += 1;
        println!("arc: {:?}", arc);
    }
}
