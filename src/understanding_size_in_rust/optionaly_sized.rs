struct Usensizedstruct<T:?Sized> {
    sized_struct: i32,
    unsized_struct: [i32],
}

fn main() {
    // create the instance of the struct
    let x = Usensizedstruct {
        sized_struct: 1,
        unsized_struct: [1],
    };
}