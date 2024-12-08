use std::sync::{Arc, Barrier, Mutex};
use std::thread;

fn main() {
    // we want to do in this program when task1 is completed, then only work on task2 in seq order

    let mut thread_vec = Vec::new();

    // for e.g., we have two tasks then we take the barrier how to work on that condition

    let tasks = Arc::new(Mutex::new(vec![]));

    // to call the barrier we need to create a new constructor function

    let barrier = Arc::new(Barrier::new(5)); // the input in the barrier indicates the number of threads need to reach the barrier

    for i in 0..5 {
        // each task is used to clone the task
        let tasks = tasks.clone();

        // we passed the clone of the barrier into each task
        let cloned_barrier = barrier.clone();

        let handle = thread::spawn(move || {
            // we are trying to gain access to the task1 and put value in the vector
            // task1
            tasks
                .lock()
                .unwrap()
                .push(format!("{0} Hello From Task 1", i));

            // to create the barrier, we need to pass the wait function on it.
            cloned_barrier.wait();

            // when the task is completely completed, then only work for the task2
            // task2
            tasks
                .lock()
                .unwrap()
                .push(format!("{0} Hello From Task 2", i));
        });
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    // to gain access to these tasks we need to first lock on it
    // to access the inner value, we need to dereference
    let take_lock = &*tasks.lock().unwrap();
    for contants in take_lock {
        println!("{}", contants);
    }
}
