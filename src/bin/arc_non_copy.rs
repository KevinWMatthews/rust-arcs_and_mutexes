// This locks out all other receivers!
/*
let arc = arc1;
let mutex = &*arc;
let result = mutex.lock();
let mutex_guard = result.unwrap();
let receiver = &*mutex_guard;
for value in receiver.iter() {
    println!("Receiver 1: '{}'", value);
}
*/

// This locks out all other receivers!
/*
let receiver = &*arc1.lock().unwrap();
for value in receiver.iter() {
    println!("Receiver 1: '{}'", value);
}
*/

fn main() {
    unimplemented!()
}
