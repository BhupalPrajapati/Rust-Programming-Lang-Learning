
                              // In this ection we are implemet the single linked list

                              /* 
#[derive(Debug)] 
// here we an define the head of linked list where the linked list is start 
// this struct is called wrapper struct
// we are also required to emptu head bcz some condtiton are empty show that why need None value that shows our head is empyt
// for that reason we need to define the Option in Linkedlist struct
struct LinlList{
    // head:Option<Node>,
    head:Pointer,
}

#[derive(Debug)]    
struct  Node{
    element:i32,
    // next:Option<Box<Node>>,
    next:Pointer,
}
// here we define the own type that is help to lead the trait
// define alis for exisitng type

type Pointer = Option<Box<Node>>;


           // here we gone to add some functionality of linked list like adding, removing and displaying 
impl LinlList  {
    fn new()->LinlList{      // here fn new() means pattrern commanly used for the instance of struct and enum this is called associated function 
        LinlList{head:None}
    }
    // here we define the adding method to add the element in the linkedlist by input is &mut of self and element we want to add

    fn add(&mut self, element:i32){
             // here we find the head have some element or none if head have no element then we create a new node and pointing to
             // if in head there some exit element then make a new node and remove head element and poting with the new node
        // match self.head {
        //     None=>{
        //         let new_node = Some(Box::new(Node
        //         {
        //             element:element,
        //             next:None,

        //         }));   
        //         self.head = new_node;
        //     }
        //     Some(Previous_head)=>{
        //         let new_node = Some(Box::new(Node{
        //             element:element,
        //             next:Some(Previous_head),
        //         }));
        //         self.head = new_node;
        //     }
        // }
               // here we are doing the some special method for the solved the above proble
               // take() method wchin is a option<> this is used when the value is missing or unknown
               // syntax of take() metod 
    //    fn take<T>(dest:&mut T)->T 

    let previous_head = self.head.take();       
    let new_head = Some(Box::new(Node{
        element:element,
        next:previous_head,
    }));
    self.head=new_head;

    }

        // here we are the remove the 1st element and alter head element

   fn remove(&mut self)->Option<i32>{
    match self.head.take() {
        Some(previous_head)=>{
            self.head=previous_head.next;
            Some(previous_head.element)
        }
        None=>None
    }
   }   

   fn print(&self){
    let mut list_traversal = &self.head;
    while !list_traversal.is_none() {
        println!("{:?}",list_traversal.as_ref().unwrap().element);
        list_traversal = &list_traversal.as_ref().unwrap().next;
        
    }
   }  
}

fn main(){

    // lets comment out the below code bcz above we are used the type pointer that used as a alias for the exsiting element
    /* 
let list = Node{
    element:1,
    next:None,
};
let list = Node{
    element:1,
    next:Some(Box::new(Node{
        element:2,
        next:None,
    }))
};
// here we are create the head of the linked list
// this provide explicit information according head/.
let list = LinlList{
    head:Some(Node{
        element:1,
        next:None,
    }),

};

// More are added int the linkedlist
let list = LinlList{
    head:Some(Node{
        element:1,
        next:Some(Box::new(Node{
            element:2,
            next:Some(Box::new(Node{
                element:3,
                next:None,
            }))
        }))
    }),
};

// we can have list with empty head
let list = LinlList{head:None};

*/

// here we arecreating the new linked list with help of the head and type pointer
/* 
let list = LinlList{
   head:Some(Box::new(Node{
    element:100,
    next:Some(Box::new(Node{
        element:200,
        next:None,
    })),
   })),
};

// here we are trying to print the head of the linkedlsit

println!("{:?}",&list.head.unwrap().next.unwrap().element);
*/

          // here we add element in the linked list with the help of the creating take method 
    let mut list = LinlList::new();
    list.add(100);
    list.add(210);
    list.add(234);
    // println!("{}",list);
    list.print();
    println!("{}",list.remove().unwrap());
}
 */

    



                   
                   /*                  // Here below we implement the doubly linked list
 use std::{cell::RefCell,rc::Rc};

#[allow(dead_code)]
#[derive(Debug)]
struct Doubly_Linklist{
    head:pointer,
    tail:pointer,
}
struct Node{
    element:i32,
    next:pointer,
    prev:pointer,
}
type pointer = Option<Rc<RefCell<Node>>>;

impl Doubly_Linklist {
    // this is the new constructor function where we can define the condtruct of the functions
    fn new()->Self{
        Doubly_Linklist{
            head:None,
            tail:None,
        }
    }

    // the add() method for the adding the elememt in the list
    // the input will be mutable reference to self and the element that we want to add

    fn add(&mut self,element:i32){
        // create new head
        let new_head = Rc::new(RefCell::new(Node{
            element:element,
            next:None,
            prev:None,
        }));

        // take will return the value of head, replacing it with default value this case is none 
        // the none value for the head will be replaced inside the match arms
        match self.head.take(){
        // we need to do two things 1)we will set the previous of the old head to new head and following thisnext of head equal to old head
        Some(old_head)=>{
            old_head.borrow_mut().prev = Some(new_head.clone());     // to muted all head we call borrow_mut() method
            new_head.borrow_mut().next = Some(new_head.clone());
            self.head = Some(new_head);
        }
        None=>{
            self.tail=Some(new_head.clone());
            self.head=Some(new_head);
        }
        }
    }
    fn remove(&mut self)->Option<i32>{
        if self.head.is_none() {
            println!("List is empty so we can't remove");
            None
        }else {
            self.head.take()
            .map(|old_head|)
        }
    }
}

fn main(){

}

*/