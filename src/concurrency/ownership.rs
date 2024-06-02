use std::thread;
fn main(){
    let x = "some value".to_string();
    println!("{}", x);
    thread::spawn( ||{
        let y = x;
        println!("{y}");
 });
}

// fn main(){
//     let x = 10;
//     println!("{}",x);
// }