/*
        ************* Design Linked List *************

Design your implementation of the linked list. You can choose to use a singly or doubly linked list.
A node in a singly linked list should have two attributes: val and next. val is the value of the current node, and next is a pointer/reference to the next node.
If you want to use the doubly linked list, you will need one more attribute prev to indicate the previous node in the linked list. Assume all nodes in the linked list are 0-indexed.

Implement the MyLinkedList class:

MyLinkedList() Initializes the MyLinkedList object.
int get(int index) Get the value of the indexth node in the linked list. If the index is invalid, return -1.
void addAtHead(int val) Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list.
void addAtTail(int val) Append a node of value val as the last element of the linked list.
void addAtIndex(int index, int val) Add a node of value val before the indexth node in the linked list. If index equals the length of the linked list, the node will be appended to the end of the linked list. If index is greater than the length, the node will not be inserted.
void deleteAtIndex(int index) Delete the indexth node in the linked list, if the index is valid.
 

Example 1:

Input
["MyLinkedList", "addAtHead", "addAtTail", "addAtIndex", "get", "deleteAtIndex", "get"]
[[], [1], [3], [1, 2], [1], [1], [1]]
Output
[null, null, null, null, 2, null, 3]

Explanation
MyLinkedList myLinkedList = new MyLinkedList();
myLinkedList.addAtHead(1);
myLinkedList.addAtTail(3);
myLinkedList.addAtIndex(1, 2);    // linked list becomes 1->2->3
myLinkedList.get(1);              // return 2
myLinkedList.deleteAtIndex(1);    // now the linked list is 1->3
myLinkedList.get(1);              // return 3
 

Constraints:

0 <= index, val <= 1000
Please do not use the built-in LinkedList library.
At most 2000 calls will be made to get, addAtHead, addAtTail, addAtIndex and deleteAtIndex.

*/

use std::ptr::eq;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub prev: Option<Weak<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            next: None,
            prev: None,
        }))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        // Compare the values of the current nodes
        if self.val != other.val {
            return false;
        }
        
        // Compare the next nodes
        match (self.next.as_ref(), other.next.as_ref()) {
            (None, None) => {}
            (Some(n1), Some(n2)) => {
                if !Rc::ptr_eq(n1, n2) {
                    return false;
                }
            }
            _ => return false,
        }
        
        // Compare the previous nodes
        match (self.prev.as_ref(), other.prev.as_ref()) {
            (None, None) => {}
            (Some(p1), Some(p2)) => {
                if !eq(p1, p2) {
                    return false;
                }
            }
            _ => return false,
        }
        
        true
    }
}

#[derive(Debug)]
pub struct DoublyLinkedList {
    pub head: Option<Rc<RefCell<Node>>>,
    pub tail: Option<Rc<RefCell<Node>>>,
}

impl DoublyLinkedList {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    

    pub fn add_at_head(&mut self, val: i32) {
        let new_node = Node::new(val);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn add_at_tail(&mut self, val: i32) {
        let new_node = Node::new(val);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_tail.clone());
                self.tail = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        // Case 1: If the index is negative, add the node at the head of the linked list.
        if index <= 0 {
            self.add_at_head(val);
            return;
        }
    
        // Traverse the linked list to find the node at the index.
        let mut current_node = self.head.clone();
        let mut current_index = 0;
        while let Some(node_ref) = current_node {
            if current_index == index - 1 {
                // Create a new node and insert it after the current node.
                let new_node = Rc::new(RefCell::new(Node {
                    val,
                    next: node_ref.borrow_mut().next.take(),
                    prev: Some(Rc::downgrade(&node_ref)),
                }));
                if let Some(next_node) = new_node.borrow_mut().next.as_mut() {
                    next_node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                }
                node_ref.borrow_mut().next = Some(new_node);
                return;
            }
            current_node = node_ref.borrow().next.clone();
            current_index += 1;
        }
    
        // Case 2: If the index is greater than or equal to the length of the linked list,
        // add the node at the end of the linked list.
        self.add_at_tail(val);
    }
    

    pub fn delete_at_index(&mut self, index: i32) {
        // Case 1: If the index is negative or greater than or equal to the length of the linked list, do nothing.
        if index < 0 {
            return;
        }
    
        let mut current_node = self.head.clone();
        let mut current_index = 0;
    
        while let Some(node_ref) = current_node {
            if current_index == index {
                if Rc::ptr_eq(&node_ref, &self.head.as_ref().unwrap().clone()) {
                    self.head = node_ref.borrow().next.clone();
                    if let Some(new_head) = self.head.clone() {
                        new_head.borrow_mut().prev = None;
                    } else {
                        self.tail = None;
                    }
                } else if Rc::ptr_eq(&node_ref, &self.tail.as_ref().unwrap().clone()) {
                    self.tail = node_ref.borrow().prev.as_ref().unwrap().upgrade();
                    if let Some(new_tail) = self.tail.clone() {
                        new_tail.borrow_mut().next = None;
                    } else {
                        self.head = None;
                    }
                } else {
                    let prev_node = node_ref.borrow().prev.as_ref().unwrap().upgrade().unwrap();
                    let next_node = node_ref.borrow().next.as_ref().unwrap().clone();
                    prev_node.borrow_mut().next = Some(next_node.clone());
                    next_node.borrow_mut().prev = Some(Rc::downgrade(&prev_node));
                }
                break;
            }
    
            current_node = node_ref.borrow().next.clone();
            current_index += 1;
        }
    }
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_at_head() {
        let mut linked_list = DoublyLinkedList::new();

        linked_list.add_at_head(1);
        assert_eq!(linked_list.head.as_ref().unwrap().borrow().val, 1);
        assert_eq!(linked_list.tail.as_ref().unwrap().borrow().val, 1);

        linked_list.add_at_head(2);
        assert_eq!(linked_list.head.as_ref().unwrap().borrow().val, 2);
        assert_eq!(linked_list.tail.as_ref().unwrap().borrow().val, 1);
    }

    #[test]
    fn test_add_at_tail() {
        let mut linked_list = DoublyLinkedList::new();

        linked_list.add_at_tail(1);
        assert_eq!(linked_list.head.as_ref().unwrap().borrow().val, 1);
        assert_eq!(linked_list.tail.as_ref().unwrap().borrow().val, 1);

        linked_list.add_at_tail(2);
        assert_eq!(linked_list.head.as_ref().unwrap().borrow().val, 1);
        assert_eq!(linked_list.tail.as_ref().unwrap().borrow().val, 2);
    }

    #[test]
    fn test_add_at_index() {
        let mut linked_list = DoublyLinkedList::new();

        linked_list.add_at_index(0, 1);
        assert_eq!(linked_list.head.as_ref().unwrap().borrow().val, 1);
        assert_eq!(linked_list.tail.as_ref().unwrap().borrow().val, 1);

        linked_list.add_at_index(0, 2);
        assert_eq!(linked_list.head.as_ref().unwrap().borrow().val, 2);
        assert_eq!(linked_list.tail.as_ref().unwrap().borrow().val, 1);

        linked_list.add_at_index(1, 3);
        assert_eq!(linked_list.head.as_ref().unwrap().borrow().val, 2);
        assert_eq!(linked_list.tail.as_ref().unwrap().borrow().val, 1);
        let second_node = linked_list.head.as_ref().unwrap().borrow().next.as_ref().unwrap().clone();
        assert_eq!(second_node.borrow().val, 3);
    }

    #[test]
    fn test_delete_at_index() {
        let mut linked_list = DoublyLinkedList::new();

        linked_list.add_at_head(1);
        linked_list.add_at_head(2);
        linked_list.add_at_head(3);

        linked_list.delete_at_index(1);
        assert_eq!(linked_list.head.as_ref().unwrap().borrow().val, 3);
        assert_eq!(linked_list.tail.as_ref().unwrap().borrow().val, 1);
        let second_node = linked_list.head.as_ref().unwrap().borrow().next.as_ref().unwrap().clone();
        assert_eq!(second_node.borrow().val, 1);

        linked_list.delete_at_index(1);
        assert_eq!(linked_list.head.as_ref().unwrap().borrow().val, 3);
        assert_eq!(linked_list.tail.as_ref().unwrap().borrow().val, 3);
        assert!(linked_list.head.as_ref().unwrap().borrow().next.is_none());
    }
}





/*
pub fn get_node(&self, index: i32) -> Option<Rc<RefCell<Node>>> {
        if index < 0 {
            return None;
        }
    
        let mut curr = self.head.as_ref();
    
        for _ in 0..index {
            match curr {
                Some(node_ref) => {
                    let node = node_ref.borrow();
                    curr = node.next.as_ref();
                }
                None => return None,
            }
        }
    
        match curr {
            Some(node_ref) => {
                let node = node_ref.borrow();
                let next = node.next.as_ref();
                next.map(|n| n.clone())
            }
            None => None,
        }
    }
*/