use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(42);
    println!("mutex: {:?}", mutex);

    {
        println!("");
        // pub fn lock(&self) -> LockResult<MutexGuard<T>>
        // LockResult is a Result type
        let lock_result = mutex.lock();

        // Extract the value from the LockResult
        let mutex_guard = lock_result.unwrap();

        // Access the member within the MutexGuard using the Deref and DerefMut traits.
        let result: i32 = *mutex_guard;
        println!("result: {}", result);
        println!("mutex: {:?}", mutex); // Data is locked!
    } // Mutex is unlocked when the mutex guard (lock result?) goes out of scope.

    {
        println!("");
        let value = *mutex.lock().unwrap();
        println!("value: {}", value);
    }

    {
        println!("");
        let lock_result = mutex.lock();
        let mut mutex_guard = lock_result.unwrap();

        // Dereference and modify the value using the DerefMut trait.
        *mutex_guard += 1;

        let value = *mutex_guard;
        println!("value: {:?}", value);
        println!("mutex: {:?}", mutex); // Data is locked!
    }

    // The Debug trait must lock and unlock the mutex internally
    println!("mutex: {:?}", mutex);

    {
        println!("");
        *mutex.lock().unwrap() += 1;        // Lock goes out of scope on this line
        println!("mutex: {:?}", mutex);
    }
}
