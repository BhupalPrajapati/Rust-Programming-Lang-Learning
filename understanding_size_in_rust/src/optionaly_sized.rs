struct Usensizedstruct<'a> {
    sized_struct: i32,
    unsized_struct: &'a [i32], // Using a reference to a slice
}

pub fn main() {
    // Create an array
    let array = [1];

    // Create an instance of the struct
    let x = Usensizedstruct {
        sized_struct: 1,
        unsized_struct: &array,
    };

    // Access fields of the struct
    println!("sized_struct: {}", x.sized_struct);
    println!("unsized_struct: {:?}", x.unsized_struct);
}
