extern crate regex;
use regex::Regex;
fn main(){
    let re = Regex::new(r"[ptr]ain").unwrap();  // this regex is called the string regex , match only one char insie the brackets
    
    // here we define the another string which has match relevent
    let text = "The rain in Spain stays mainly in the plain";
    
    // here we use match function to find the match in the text 
    println!("The text has a match:{:?}",re.is_match(text));


}