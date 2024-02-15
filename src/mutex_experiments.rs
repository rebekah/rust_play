use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let counter2 = Arc::clone(&counter);

    let handle = thread::spawn(move || {
        let mut value = counter2.lock().unwrap();
        *value += 1;
    });

    handle.join().expect("Thread panicked");

    let value = counter.lock().unwrap();
    println!("Counter: {}", *value);
    *value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run(), 1);
    }
}
