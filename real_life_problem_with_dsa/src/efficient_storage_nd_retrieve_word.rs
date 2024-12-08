// Efficient storage and retrieval of words
// consider a business , system want to desgin that involves the search of particular or fetching the word in a efficiently torage

// we are implement this problem by using the trie data structure
// What is the trie data structure?
// Ans = The trie data structure is also known as the digital tree or prefix tree. it is type of k array search tree which is used to for locating specific keys from within a set
// How you used trie data structure in this problem
// Ans = we have trie data structure which have node and parent and child, if we are follow from the node we go them in child .
// structure is :
//      root
//      /   \
//     a    t
//    /  |  |
//   e   i  n
//   / \   \
//  s   o   g
// struct Node{
// children: HashMap<char, Node>,
// is_word: bool,
//}
// there two children
// Root
//Node:children  <T,Node> <A,Node> is_word=false

// lets define the data structure
// we use the hashmap for the fast stroge and traversing
// and the vector for storing the word
use std::collections::HashMap;

// next use the driving the neccessary properties for the node
#[derive(Defult, Debug, PartialEq, Eq, Clone)]
struct Node {
    children: HashMap<char, Node>,
    is_word: bool,
}
// implement a function which is create the new node
impl Node {
    // insie the function ww will
    fn new() -> Self {
        Node {
            is_word: false,
            children: HashMap::new(),
        }
    }
}
// lets create a wrapper around the node which will refer to as a dictionary
// we drive the relevent properties
#[derive(Debug, Defult, PartialEq, Eq, Clone)]
// now define the word dictionary struct word
struct WordDictionary {
    // it has ainle field of root, which is type Node
    root: Node,
}

// we will now implement this dictionary by using implementations block
impl WordDictionary {
    // as usual first we define the function creating the new dictionary

    fn new() -> Self {
        // jst call the Defual function which is makes the call default values,word dictionary only contains single filed of root, whic is node. so that default of node will be assign
        // which is comes from the new function of the node
        Self::default()
    }
    // now declare the insert function
    // this function takes one word at a time and inserts it into tree
    // the input will be immtauble reference to the self and the word that we want to insert inside the function
    fn insert(&mut self, word: &String) {
        // declare a variable current which is initialized from the root.
        let mut current = &mut self.root;
        // for each character in the word
        // for each letter we will insert it into the HashMap as a key and the values part of new Node

        for w in word.chars() {
            current = current.children.entry(w).or_insert(Node::new());
        }
        // if we done insertion of word in tree, then need to update the last node in the tree
        if !current.is_word {
            current.is_word = true;
        }
    }

    // now look the implementation of search function
    fn search(&self, word: &String) -> bool {
        // the input of function will be a reference to self and the word that we want search in the dictionary
        let mut current = &self.root;
        // iterate through each letter of the word a
        for w in word.chars() {
            // determine of nodes have some key in the given hashmap of given childern
            if current.children.get(&w).is_some() {
                // if we found the key then we move to the next node
                current = current.children.get(&w).unwrap(); // unwrap used used because the get function return the an Option
                                                             // if none of child key equal to letter then we will return a false bcz it means that word is not present in the entire di
            } else {
                return false; // if the loop return the successfully value without any fails , then we return last processing node in the tree.;
            }
        }
        current.is_word
    }
}

fn main() {
    let words = vec![
        "the", "a", "there", "answer", "any", "by", "bye", "their", "abc",
    ]
    .into_iter()
    .map(|x| String::from(x))
    .collect::<Vec<String>>();

    // call function

    let mut d = WordDictionary::new();
    for i in 0..words.len() {
        d.insert(&words[i]);
    }
    println!(
        "Searching 'there' in the dictionary results:{}",
        d.search(&"there".to_string())
    );
}
