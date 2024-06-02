fn main(){
    // find the frit start with a and b also all word is convertred in uppercase
    let words = vec!["apple","banana", "orange","mango"];
    // creatinf the empty vector string to storing the final result

    // let mut result = vec![];
    // for word in words{
    //    if word.starts_with("a") || word.starts_with("b"){
    //     // we converted the words in uppercase

    //     let upper_case = word.to_uppercase();
    //     // and finally added to the vector

    //     result.push(upper_case);
    //    }
    // }
    // println!("{:?}",result );


        // The above things are done with help of the combinator which is works same as above

    let result  = words
     .into_iter()
     .filter(|&word|word.starts_with("a") || word.starts_with("b"))
      .map(|word|word.to_uppercase())
      .collect::<Vec<String>>();

    println!("{:?}",result );
}