pub mod Regular_Exp_Basics;
pub mod basic_file_handling;
pub mod director_and_path;


extern crate regex;
use std::process::Termination;

use regex::Regex;
fn main() {
    // println!("Hello, world!");



                             // Here we are doing Regular_Exp_Basics folder works
/* 
    let re = Regex::new(r"[ptr].ain").unwrap(); // this regex is called the string regex , match only one char insie the brackets

    // here we define the another string which has match relevent
    let text = "The rain in Spain stays mainly in the plain";

    // here we use match function to find the match in the text
    // println!("The text has a match:{:?}", re.is_match(text));

    // // letd use the find() fun on it, which return thr where it is started and where it is ended
    // println!("The text has a match {:?}",re.find(text));

    // here we using capture() fucntion to match the individual TEXT 
    for cap in re.captures_iter(text){
        println!("{:?}", cap);
    }
   */



//   let re = Regex::new(r"gr[ae]y").unwrap();
//   let text = "gray, grey, gay, gaying";

//   for cap in re.captures_iter(text) {
//     println!("{:?}",&cap[0]);
//   }

//   let re = Regex::new(r"[a-z]ain").unwrap();
//   let text = "The rain in Spain stays mainly in the plain";
//   for cap in re.captures(text){
//     println!("match:{:?}",&cap[0]);
//   }


/* 
  // lets write another regex and try to match the input number with that regex
  let re = Regex::new(r"\d\d\d\d\d\d").unwrap();
//   let text = "we have 8888888 rupes for 100000 days";
  let text = "999999";
  for c in re.captures_iter(text){
    println!("{:?}", c);
  }
  */



/* 
  let re = Regex::new("^abc").unwrap();
  let text = "abca123";
  // here want to print the matches
  for c in re.captures_iter(text){   // cap is used for the match the input at start only

    println!("{:?}", &c[0]);
  }

  */

          // Matching the anchor 
/* 
    // let re = Regex::new(r"bc$").unwrap();
    let re = Regex::new(r"^bc$").unwrap();
    let text = "aba abc bc";
    for c in re.captures_iter(text)  {   // the anchoring tag is start to the match from the last chara and find the regex with input value``
        println!("{:?}", &c[0]);
        
    }    
*/


// let re = Regex::new(r"^\d\d$").unwrap();
// let text = "12";
// for c in re.captures_iter(text)  {
//     println!("{:?}", &c[0]);
// }

// let re = Regex::new(r"\b\w*").unwrap();
// let text = "The quick brown fox jumps over the lazy dog";
// for c in re.captures_iter(text)  {
//     println!("{:?}", &c[0]);
// }



                                    







                                    // Regexes- Repeatitons and Quantifiers

   // There are three types of quantifiers that are commonly used called the question mark, a plus and a star
      // The question(?) mark is indicated the 0 or 1 times of repeatitons                                 
      // The Plus(+) is used to indicate the 1 0r more timed of repeatitons
      // the star(*) is the used to indicate 0 or more times of repeatitons

// let re = Regex::new(r"a?aa").unwrap();
// let text = "aa aaa";

// let re = Regex::new(r"bsa?").unwrap();
//  let text = "b ab ssabssa ba";


// let re = Regex::new(r"\w?\w?\w?\w?\w?.rs").unwrap();  // this is show .rs file contains
// let text = "main.rs lib.rs src/main.rs";
// for c in re.captures_iter(text)  {
//     println!("Match :{}", &c[0]);
// }


// let re = Regex::new(r"a+").unwrap();
// let text = "aaa,  bba, aaba, aacv, aadc";
// for c in re.captures_iter(text)  {
//     println!("Match :{}", &c[0]);
// }


// let re = Regex::new(r"\w+\.gif").unwrap();
// let text = "image.gif and background.gif";


// let re = Regex::new(r"ab*").unwrap();
// let text = "ab, aba, abaaa, abbbbbb";
// for c in re.captures_iter(text)  {
//     println!("Match :{}", &c[0]);
// }




    
           // if you want to control the repetition of multiple times you can write the code acording the below

        // //    let re = Regex::new(r"\b\w{2,10}\b").unwrap();
        // let text = "I think you we are happy because i have gift for you";

           let re = Regex::new(r"\d{1,3}\.\d{1,3}").unwrap();
           let text = "192.168.0.1 192.168.0.255 127.0.0.1 10.0.0.1";
           for c in re.captures_iter(text)  {
               println!("Match :{}", &c[0]);
           }


}
