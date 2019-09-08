use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let arc = Arc::new(Mutex::new(0));

    // Could clone only once if we don't need to access the shared data again
    let arc1 = Arc::clone(&arc);
    let arc2 = Arc::clone(&arc);

    let handle1 = thread::spawn(move || {
        println!("arc1: {:?}", arc1);

        let subarc = Arc::clone(&arc1);
        // Experiment with remove the 'move' keyword -
        // Then the borrowed value doesn't live long enough
        let subhandle = thread::spawn(move || {
            println!("subarc: {:?}", subarc);
            *subarc.lock().unwrap() -= 7;
        });

        *arc1.lock().unwrap() += 1;
        subhandle.join().unwrap();
    });
    let handle2 = thread::spawn(move || {
        println!("arc2: {:?}", arc2);
        *arc2.lock().unwrap() += 2;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("arc: {:?}", arc);
}
