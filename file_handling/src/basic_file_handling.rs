use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::{fs::*, path};

fn basic_file_handling() -> std::io::Result<()> {
    let path_loc = r"C:\Users\GS-3935\Documents\my_text.txt";
    let path = Path::new(&path_loc);

    /*
        let mut file = File::create(path)?; // this function is rerurn the resul, therefore we used the question mark here to return the result

                                        // now use the write function i file to write something in the file

        file.write_all(b"Hello, World!")?;
        file.write("How are you".as_bytes())?;

        // here we are append the file after writing in the previous file
        let mut file = OpenOptions::new().append(true).open(path)?;

        // before run this below scrpit comment out the all previous scrpit. because it is overwrite the existing file file in the disk
        file.write("\n www.inculdeact.com\n".as_bytes());

        let str1 = "Hello";
        file.write(str1.as_bytes());

        let some_vec = vec![1, 2, 3, 4, 5];
        let str_from_vec = some_vec
            .iter()
            .map(|a| format!("{} \n", a.to_string()))
            .collect::<String>(); // we collect in from of the string

        file.write(str_from_vec.as_bytes());

        let(name,age) = ("School",12);
        let formated_str = format!("I am {} my name is {}",name,age);
        file.write(formated_str.as_bytes())?;


    */

    // Reading the contents from the file

    // let mut file = File::open(path)?;
    // let mut contents = String::new();

    // // read the contents of the file

    // file.read_to_string(&mut contents)?;

    // // print the contents

    // println!("The file contains{:?}",contents);

    let mut file = File::open(path)?;
    let file_buffer = BufReader::new(file);
    for lines in file_buffer.lines() {
        println!("{}", lines?);
    }

    Ok(())
}

fn main() {
    basic_file_handling();
}
