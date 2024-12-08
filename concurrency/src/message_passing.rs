// messagin gpasssing is done in rust by the mpsc channel(multiple producers and single consumer    )

use std::{sync::mpsc, thread};
fn main() {
    let (tx, rx) = mpsc::channel();

    // let using the muilple value , for that using the loops
    for i in 0..5 {
        // for the using of multiple threads, you need to create the clone of that varibale
        // here we are trying to create the clone of the tx variable

        let tx_clone = tx.clone();
        thread::spawn(move || {
            let val = String::from("hi");
            println!("Send Message :{val}");
            tx_clone.send(val).unwrap();
            // when 1st time message/thread is send is send then it is not available again

            // println!("{val}");   // there is show error in that condition, show no longer avialbe that vl bcz it is avl bcz it is already moved
        });
    }

    // Note
    // The main thread of the transmiter is runing in the system, so we need to the drop the that thread after the receiver is end

    drop(tx);

    /*
    let receiver_value = rx.recv().unwrap(); // that line is onky received the only one message
    println!("Received Message :{}",receiver_value);

    // adding the another line to receive the messsage for the another thread

    let receiver_value = rx.recv().unwrap();
    println!("Received Message :{}",receiver_value);
    */

    // to received all the message we need to treat the receiver like the iterator

    for message in rx {
        println!("Received Message :{}", message);
    }
}
