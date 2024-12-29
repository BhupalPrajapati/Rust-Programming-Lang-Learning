// reference refreference



fn main() {
    let mut vec_1 = vec![1, 2, 3];
    let mut vec_2 = vec![4, 5, 6];
    let mut reference = &vec_1;

    let mut reference = &mut vec_1;
    reference.push(4);
    println!("{:?}", reference);
    reference = &mut vec_2;

    let mut reference = &mut vec_1;
    reference.push(4);
    reference = &mut vec_2;
    println!("{:?}", reference);

    // lets talk about the mutable reference and immutable reference

    // Immutable reference not moved but copied
    let name1 = String::from("John");
    let ref1 = &name1;
    let ref2 = &name1;
    println!("ref1: {}", ref1); // this is copied not moved from exact location, because it is stored in the stack

    // Mutable reference moved but not copied

    let mut name2 = String::from("John");
    let mut ref3 = &mut name2;
    // let ref4 = &mut name2;
    println!("ref1:{}", ref3); // this is moved from exact location, because it is stored in the heap

    // ref_slice
    // a reference is to slice have static life time
    // for eg

    let str = String::from("value");
    let mut str_slice1: &str;
    {
        let str_slice2 = &str[..];
        str_slice1 = str_slice2;
    }
    {
        let str_slice2 = "";
        str_slice1 = str_slice2;
    }
    // here i am able to access the slice1
    println!("str_slice1: {}", str_slice1);
    println!("{str_slice1}");
    // but i can't access the slice2 because it is moved to the heap
    //  println!("str_slice2: {}", str_slice2);

    // Dereference concept

    let str_slice_string: &str;
    {
        let str2 = String::from("value");
        str_slice_string = &*str2;  // it is no longer available outside the loop
    }
    // println!("{str_slice_string}");


    // basically the Dereference is used to convert the immutable ref to mutable ref
    // for eg
    let mut x =    10;
    let z = &mut x;
    let y = &*z;
    let a = & *y;
    
    println!("{y}");


}
