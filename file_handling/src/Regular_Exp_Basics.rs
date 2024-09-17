
fn main(){
    // let re = Regex::new(r"[ptr]ain").unwrap();  // this regex is called the string regex , match only one char insie the brackets
    
    // // here we define the another string which has match relevent
    // let text = "The rain in Spain stays mainly in the plain";
    
    // // here we use match function to find the match in the text 
    // println!("The text has a match:{:?}",re.is_match(text));

    




                           // String Iterals

//  A "string literal" is a sequence of characters from the source character set enclosed in double quotation marks ( " " )                          

    let sa = r#"The main role \" Police offcier\""#   ; // where r means raw string and # is the starting of the string and ending of the string

    // the main used of the string Iterals is used for the JSON representation

    // JSON representation
    let json = "{
    \"name\":\"Bhupal\",
    \"age\":23,
    \"height\":5.6 feet,
    \"hobbies\": [\"Reading\", \"Gaming\", \"Dancing\"]
    }";


    // this above json representation we can also define in the following ways
    let json2 = r#"{
    "name":"Bhupal",
    "age":23,
    "height":5.6,
    "hobbies":["Reading", "Gaming", "Dancing"]
    }"#;

    println!("{}", json2);
 // println!("{}", re.is_match(text));
 // println!("{}", sa);
    println!("{}", sa);
    println!("{}", json);
}