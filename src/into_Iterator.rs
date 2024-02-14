                        // The main advantes of into iterator lies is in ability to allow a type to be used in a for loop
                        //other iterator consuming context




struct Book{
    title:String, 
    author : String,
    genre: String,
}


// struct BookIterator{
//     properties:Vec<String>,
// }

// // lets implement the build itertor for the Book which is store the item for the book
// impl Iterator for BookIterator{
//     type Item = String;
//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.properties.is_empty(){
//             Some(self.properties.remove(0))
//         }else {
//             None
//         }
//     }

// }

// Lets implement the BookIterator fot the Book

impl IntoIterator for Book {
    type Item = String;
    // it gives instance, we should able to convert it inot some type which implement the iterator 


    // type IntoIter = BookIterator;  //(one thing note we are implemenitnf the book iterator so the type into for bookIterator is not requored)
     
// finally we will update the into iter to that of a vector iterator  with the type of self item

type IntoIter = std::vec::IntoIter<Self::Item>;

    // it will simply return an instnces of the Book iterator
    fn into_iter(self) -> Self::IntoIter {
        // BookIterator{
        //     properties:vec![self.title, self.author, self.genre],
        // }


        // instead of returning book iterator we are returning the vec iterator
       vec![self.title, self.author, self.genre].into_iter()
        
    }
    
}
fn main(){
let book1 = Book{
    title:"MyBook".to_string(),
    author:"Manxe".to_string(),
    genre:"English".to_string(),
};
// next we cal the intoItertor on Book, which is convert it into iterator

let  book_iterator = book1.into_iter();
// the into method returns the book ierator instances on which we can call next method to ierator over
// println!("{:?}", book_iterator.next());
// println!("{:?}", book_iterator.next());
// println!("{:?}", book_iterator.next());
// println!("{:?}", book_iterator.next());

   // from the main advantages of the into iterator so we can easly used for loop

for book_info in book_iterator{
    println!("{book_info}");
}   

}

