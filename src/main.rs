use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)] // Add PartialEq here
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn push_node(&mut self, node: Node<T>) {
        let mut new_node = Box::new(node);
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    pub fn has_loop(&self) -> bool {
        let mut tortoise: &Option<Box<Node<T>>> = &self.head;
        let mut hare: &Option<Box<Node<T>>> = &self.head;

        while hare.is_some() && hare.as_ref().unwrap().next.is_some() {
            tortoise = &tortoise.as_ref().unwrap().next;
            hare = &hare.as_ref().unwrap().next.as_ref().unwrap().next;

            if let (Some(t), Some(h)) = (tortoise, hare) {
                if std::ptr::eq(t, h) {
                    return true;
                }
            }
        }

        false
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = LinkedList::new();
        let mut current = &self.head;

        while let Some(node) = current {
            let cloned_node = Node {
                value: node.value.clone(),
                next: None, // This will be updated in the next iteration
            };
            new_list.push_node(cloned_node);
            current = &node.next;
        }

        new_list
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);
    list.push(2);

    let cloned_list = list.clone();

    if let Some(ref mut head) = list.head {
        head.next.as_mut().unwrap().next = Some(Box::new(Node {
            value: 1,
            next: Some(Box::new(*cloned_list.head.unwrap())), // Use the cloned list head
        }));
    }

    if list.has_loop() {
        println!("The linked list has a loop.");
    } else {
        println!("The linked list does not have a loop.");
    }
}
