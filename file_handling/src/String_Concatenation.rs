fn main(){
    // let s1 = String::from("Hello");
    // let s2 = "World";
    // let s3 = s1+s2;
    // println!("{}", s3);


    // another form of the string where we can accept the both string and con

    // let s1 = String::from("Hello");
    // let s2 = String::from("World");
    // let s3 = s1+&s2;
    // println!("{}", &s3);



    // another example where we can concatenate three strings

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = String::from("!");

    let s4 = s1+&s2+&s3;
    println!("{}", &s4);
}