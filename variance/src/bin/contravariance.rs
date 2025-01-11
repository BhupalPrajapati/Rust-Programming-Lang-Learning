


// but the compiler need to static lifetime

// for the static lifetime i have defined here a functional pointer, who took the static lifetime of the variable
// fn(fn(&'a str)) ->()  // it si called the functional pointer, but the function pointer a shorter lifetime except so that compiler will give the error



// the code is compiled if we provide another functional with the non-static lifetime
// fn(fn(&'a str)) ->()  
fn accept_fn(s:&'static str){
    println!("{}", s);
}
fn main(){
    println!("Contravariance");
}