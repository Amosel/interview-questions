use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let data_clone = Arc::clone(&data);
    thread::spawn(move || {
        let mut data = data_clone.lock().unwrap();
        data[0] += 1;
    })
    .join()
    .unwrap();

    println!("{:?}", *data.lock().unwrap());
}
