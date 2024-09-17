mod Employee_with_No_Meeting;
mod Highest_stock_price;
mod Product_popularity;
mod longest_nonstop_working_hrs;
mod item_sugg_using_hashset;
pub mod item_in_range_Using_Binary_tree;
pub mod my_sql;
pub mod findling_top_products;
pub mod efficient_storage_nd_retrieve_word;
pub mod Most_recently_used_product;
pub mod Displaying_Participants_in_online_meething;

use std::collections::HashMap;



   // Search Result with word grouping by using the hashmaps and linklists
   // we are implementing a problem like in a businesess store user are searchING any item with something wrong spelling
    // then also display the name of the item

    // for that 1st we split the words and implement the anagram, anagram means present same word in different positions

          
   
   fn word_grouping(words_list: Vec<String>)->Vec<Vec<String>>{
         // 1st we create a empty hashmap
         let mut word_hash = HashMap::new();

         // for culculting the frequency first we need to create the vector containing the 26 entries
         let mut char_freq = vec![0;26];
        // we iterate the whole word in the that list
        for current_word in words_list{
            // convert the lower case and grap the char inidivisale char of word
            for c in current_word.to_lowercase().chars(){

                // i willl update the frequency of current character
                char_freq[(c as u32 - 'a' as u32) as usize] += 1;  // the c as u32 convert the into ASCII from the current character
                                                                   // and then subtracting the ASCII from the letter 'a'. ASCII pf a is 97, so we subtract and get the value 3 
                
            }

            // there we convert the each char to string. map functionis used to convert to that
            let key = char_freq.into_iter().map(|i|i.to_string()).collect::<String>(); // here the key is used for the hashmap and the 
            word_hash.entry(key).or_insert(Vec::new()).push(current_word);

            char_freq = vec![0;26];
        }

        // here i am displaying the the hashmap of that
        for(key, value) in &word_hash{
            println!("key #{:?} value: {:?}", key, value);
        }

        // here i am return the values of that
        word_hash.into_iter().map(|(_,v)|v).collect()  // it take entries of all hash map and return only the value part

         
   }
fn main() {
    println!("Hello, world!");
    let words = vec!["the".to_string(), "teh".to_string(),"het".to_string(),"stupid".to_string(),"sutpdir".to_string(),"apple".to_string(),"alpep".to_string()];

    let grouping = word_grouping(words);

    // let create a varibale in which user can store the words
    let input_word = String::from("teh");

    // iterate each gropup and check in which the that word is stored
    for i in grouping.into_iter(){
        if i.contains(&input_word) {
            println!("The Suitable word is {:?}",i);
    }

}
}
