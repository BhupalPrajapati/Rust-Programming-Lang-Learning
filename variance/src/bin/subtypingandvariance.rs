//        variance in the rust

// the subtyping in rust is called the variance, for eg x:y => x is subtype of y
// variance is the relationship between the types of the different elements in the same type

// there are three types of variance
// 1. Covariance   if X':'y then T<'x> : T<'y> we can provide, compiler has no issue in this case
// 2. Contravariance
// 3. Invariance




fn accept_str(s: &str) {
    println!("{}", s);
}
fn main() {
    let s1 = String::from("Hello Dear");
    // let s2 = "";                // string slice has the static lifetime => when ever the program is running it it on there
    // // creating a raw string slice
    // let s4: &str;
    // {
    //     let s2 = "";
    //     let s4 = s2;
    // }
    // println!("{s4}"); // it is accessed the outside of the loop
    let s3 = &*s1; // s3 has not static lifetime bcz s1 is stored in the heap and when it is ging to out of scope then it is not present for the s3
    accept_str(s3);
}
