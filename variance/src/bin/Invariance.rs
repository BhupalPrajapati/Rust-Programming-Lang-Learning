//  if 'x: 'y then T<'x>: T<'y> // T is covariant in 'x and contravariant in 'y

// Invariance means that you must provide somethings which is exactly the same as the what is being asked for
// However, they covariant on there life time

// fun expexts > fn(&mut vdc<&'a str>  ,  &'a str) -> ()
// we provide > fn(&mut Vec<&'static str>  ,  &'static str) -> ()

fn accept_fn(str_vec:&mut Vec<&str>,s:&str){
    str_vec.push(s);
    println!("{}", s);
}
fn main(){
    let mut vec: Vec<&'static str> = vec!["hello", "world"];
    let non_static = &*String::from("");

}