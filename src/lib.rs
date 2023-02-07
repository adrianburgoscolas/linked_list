//! # Linked List
//!
//! `singly_linked_list` crate implements a singly linked list


use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;

/// `List` struct is the singly linked generic data type
#[derive(Debug)]
pub struct List<T: Clone> {
    val: Option<T>,
    next: Option<Rc<RefCell<List<T>>>>,
    tail: Option<Rc<RefCell<List<T>>>>,
}

impl<T: Clone> List<T> {

    /// Creates a new singly linked list
    ///
    /// # Example:
    ///
    /// ```
    /// use singly_linked_list::List;
    ///
    /// let mut list: List<u32> = List::new();
    /// assert_eq!(list.traverse(), Vec::new())
    /// ```
    pub fn new() -> List<T> {
        List {
            val: None,
            next: None,
            tail: None,
        }       
    }

    /// Adds a new item to the list
    ///
    /// # Example
    ///
    /// ```
    /// use singly_linked_list::List;
    ///
    /// let mut list: List<char> = List::new();
    /// list.add_item('a');
    /// assert_eq!(list.traverse(), vec!('a'));
    /// ```
    pub fn add_item(&mut self, value: T) {
        if let Some(_) = self.val{
            let node = Rc::new(RefCell::new(List{val: Some(value), next: None, tail: None}));
            match self.next.as_ref() {
                Some(_) => (),
                None => self.next = Some(Rc::clone(&node)),

            };
            match self.tail.as_ref() {
                Some(t) => t.borrow_mut().next = Some(Rc::clone(&node)),
                None => self.tail = Some(Rc::clone(&node)),

            };
            self.tail = Some(Rc::clone(&node));
        } else {
            self.val = Some(value);
        }
    }

    /// Traverse the hole list and returns a vector
    /// containing the list's values
    ///
    /// # Example
    ///
    /// ```
    /// use singly_linked_list::List;
    ///
    /// let mut list: List<f32> = List::new();
    /// list.add_item(2.3);
    /// list.add_item(4.3);
    /// list.add_item(8.0);
    /// list.add_item(5.6);
    /// let list_values = list.traverse();
    /// assert_eq!(list_values, vec!(2.3, 4.3, 8.0, 5.6));
    /// ```
    pub fn traverse(&self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        if let Some(_) = self.val{
            result.push(self.val.clone().unwrap());
            if let Some(_) = self.next {
                let mut next = Rc::clone(self.next.as_ref().unwrap());
                'finish: loop {
                    result.push(next.borrow().val.clone().unwrap());
                    match next.borrow().next.as_ref() {
                        Some(_) => (),
                        None => break 'finish,
                    }
                    let node = Rc::clone(next.borrow().next.as_ref().unwrap());
                    next = node;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list: List<u32> = List::new();
        assert_eq!(list.traverse(), Vec::new());
    }

    #[test]
    fn test_list_with_one_value() {
        let mut list: List<char> = List::new();
        list.add_item('a');
        assert_eq!(list.traverse(), vec!('a'));
    }

    #[test]
    fn test_list_with_multiple_values() {
        let mut list: List<f32> = List::new();
        list.add_item(2.2);
        list.add_item(5.1);
        list.add_item(0.7);
        list.add_item(3.4);
        list.add_item(4.3);
        assert_eq!(list.traverse(), vec!(2.2,5.1,0.7,3.4,4.3));
    }
}
