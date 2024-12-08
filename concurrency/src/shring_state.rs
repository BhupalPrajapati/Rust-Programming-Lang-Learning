/*
use std::sync::Mutex;
use std::thread;
fn main(){
    let x = Mutex::new(5);
    {
        let mut num = x.lock().unwrap();
        *num =10;
        println!("{num}");
    }
   let lock_m = x.lock().unwrap();
   println!("{}",*lock_m);

}
*/

// consider a file which need to accessed to the multiple files

use std::thread;
//  use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
struct File {
    text: Vec<String>,
}
fn main() {
    // create the instance of the file
    let file = Arc::new(Mutex::new(File { text: vec![] })); // The mutex is insure the text is updated by single thread at any given time

    // create thread handle to strore the thread vec
    let mut thread_vec = vec![];

    for i in 0..10 {
        // let make the clone of the file
        let file = Arc::clone(&file);

        let handle = thread::spawn(move || {
            // first acquire the lock
            let mut file_lock = file.lock().unwrap();
            file_lock.text.push(format!("Hello{i}"));
        });

        // finally add the thread to the thread vector
        thread_vec.push(handle);
    }

    // make sure when you goes for the completion of the all threads, we need to the join the these thread

    for handle in thread_vec {
        handle.join().unwrap();
    }

    // to access the file log, you need to call the lock the file
    let file_lock = file.lock().unwrap();
    println!("{:?}", file_lock.text);
}
