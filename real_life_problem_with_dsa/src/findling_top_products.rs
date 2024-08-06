// Finding the top products of the company
// description
// we are given link list corresponding to top ranked products in different countries
// we need to find the all combine these link list into consolidated link list containing the rank in an ascendin order

use std::sync::Arc;

#[derive(Debug)]
struct Linkedlist<T: std::fmt::Debug> {
    head: Pointer<T>,
}

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Pointer<T>,
}
type Pointer<T> = Option<Box<Node<T>>>;

impl<T: std::fmt::Debug> Linkedlist<T> {
    // this method will merge the two link list
    fn create_empty_list() -> Linkedlist<T> {
        Linkedlist { head: None }
    }

    fn add(&mut self, element: T) {
        let previous_head = self.head.take();
        let new_head = Box::new(Node {
            element: element,
            next: previous_head,
        });
        self.head = Some(new_head);
    }
    fn remove(&mut self) -> Option<T> {
        let previous_head = self.head.take();
        match previous_head {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.element)
            }
            None => None,
        }
    }
    fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(H) => Some(&H.element),
            None => None,
        }
    }
    fn printing(&mut self) {
        let mut list_traversal = &self.head;
        println!();

        while true {
            match list_traversal {
                Some(node) => {
                    print!(" {:?}", node.element);
                    list_traversal = &list_traversal.as_ref().unwrap().next;
                }
                None => break,
            }
        }
    }
    // here we are implement the reverse linked list traversal

    fn reverse(&mut self) {
        // 1st check the empty head
        if self.head.is_none() || self.head.as_ref().unwrap().next.is_none() {
            return;
        }

        let mut prev = None;
        let mut current_node = self.head.take();
        // let iterate the current node has some values
        while current_node.is_some() {
            let next = current_node.as_mut().unwrap().next.take();
            current_node.as_mut().unwrap().next = prev.take();
            prev = current_node.take();
            current_node = next;
        }
        // after reverse the list we set the head of the original list to the new head
        self.head = prev.take();
    }
}

// lets create a function which store these list in a vector form and return the sorted value in a consolated vector

fn sort_list(vec_list: &mut Vec<Linkedlist<i32>>) -> Linkedlist<i32> {
    // 1st we create a empty list that store the combined list value in a sorted order
    let mut sorted_list = Linkedlist::create_empty_list();
    // also i need vector of value for the my computation
    let mut values: Vec<i32> = Vec::new();

    while true {
        let values = vec_list
            .into_iter()
            .map(|x| x.head.as_ref().unwrap().element)
            .collect::<Vec<i32>>();

        // then we return the mini vakue from the vector

        let min_val = *values.iter().min().unwrap(); // min() function return the Option so that need to unwrap
        let min_index = values.iter().position(|x| *x == min_val).unwrap(); // position gives the eaxt index of the value

        // return the finall value of list by using the add function
        sorted_list.add(min_val);
        vec_list[min_index].remove(); // remove the minimum value from the original list

        // next we gone to check removed value is empty or not
        if vec_list[min_index].head.is_none() {
            // head is noe , then we go for the remove the list from the vector of list
            vec_list.remove(min_index);
        }
        // finally we are stop the iteration if we have no more lists
        if vec_list.len() == 0 {
            break;
        }
    }

    sorted_list
}

fn main() {
    let mut list1 = Linkedlist::create_empty_list();
    // add some value in that linked list in ascending order
    list1.add(45);
    list1.add(40);
    list1.add(35);
    list1.add(30);
    list1.add(23);

    let mut list2 = Linkedlist::create_empty_list();
    list2.add(85);
    list2.add(75);
    list2.add(65);
    list2.add(55);

    let mut list3 = Linkedlist::create_empty_list();
    list3.add(105);
    list3.add(95);
    list3.add(80);

    // lets call the function sorted_list and store the results in a suitable variable

    let mut result = sort_list(&mut vec![list1, list2, list3]);
    // also print the results combine three linked list
    result.printing();

    // for the reverse list, we can call here the reverse function

    result.reverse();
    result.printing();
}
