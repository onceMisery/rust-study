use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn add_with_thread(left: i32, right: i32) -> i32 {
    thread::spawn(move || left + right)
        .join()
        .expect("worker thread should finish")
}

pub fn sum_with_channel(values: Vec<i32>) -> i32 {
    let (sender, receiver) = mpsc::channel();

    for value in values {
        let sender = sender.clone();
        thread::spawn(move || {
            sender.send(value).expect("receiver should be alive");
        });
    }

    drop(sender);
    receiver.iter().sum()
}

pub fn shared_counter(workers: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let mut handles = Vec::with_capacity(workers);

    for _ in 0..workers {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut guard = counter.lock().expect("mutex should not be poisoned");
            *guard += 1;
        }));
    }

    for handle in handles {
        handle.join().expect("worker thread should finish");
    }

    let total = *counter.lock().expect("mutex should not be poisoned");
    total
}
