use std::env;
use std::fs;
use std::fs::Permissions;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    /*
        let path = Path::new(r"C:\Users\GS-3935\Documents\my_text.txt");
        println!("Printing the path of dir:{:?}", path.parent().unwrap());

        println!("Name of the file is {:?}", path.file_stem().unwrap());
        println!("Extension of the file {:?}", path.extension().unwrap());








        // i want to create some files in drive
        //i can add folders or sundir to this path by simplying using the push function

        let mut path = PathBuf::from(r"C:\");
        path.push("Users");
        path.push("GS-3935");
        path.push("Documents");

        // now add the file name
        path.push("my_new_file");

        // now add the extension of files
        path.set_extension("txt");

        // lets print the full path now
        println!("Full path: {:?}", path);







        // the above things can also be done with vector and iterators

        // lets forst create the vector of strings

        let path = [
            r"C:\",
            r"Users",
            r"GS-3935",
            r"Documents",
            r"my_new_file.txt",
        ]
        .iter()                                       // iterate the each string from the vector
        .collect::<PathBuf>();                                      // collect in the pathBuf
        println!("Full path of the folder using the vector is :{:?}", path);



    /*
        // Check some folder which are present in a some directory or not

        let path = Path::new(r"C:/Users/GS-3935/Documents");
        print!("Is the Path Directory:{:?}",path.is_dir());  // if present then is_dir() function return otherwise false;
    */



        // check the some files point to a directory or not
        // let path = Path::new(r"C:\Users\GS-3935\Documents\my_new_file.txt");
        // println!("Is present in directory :{:?}", path.is_file());




        // Now check the more details about the files like how much memory occupied and when it is created like that
        // for that we have a function is called metadata(), which gives us the more details about the files

    let data = path.metadata().unwrap();
    println!("type: {:?}", data.file_type());
    println!("length : {:?}", data.len());
    println!("permissions : {:?}", data.permissions());
    println!("Modified : {:?}", data.modified());
    println!("Created: {:?}", data.created());



    */

    // more function about the files and directories

    // from this we can see the total files contains on this location/driver
    //   let path = Path::new(r"C:\Users\GS-3935\Documents");
    //   for file in path.read_dir().expect("read_dir call failed"){
    //     println!("{:?}", file.unwrap().path());
    //   }

    // get the current files in the driver

    let mut curr_path = env::current_dir().expect("can't find current directory");
    println!("{:?}", curr_path);

    // create new directory
    //  println!("Creating a new directory:{:?"fs::create_dir(r"C:\test"));

    // for remove the directory
    // println!("Removing the directory: {:?}", fs::remove_dir(r"C:\test"));
}
