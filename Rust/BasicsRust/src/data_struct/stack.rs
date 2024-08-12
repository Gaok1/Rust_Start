use core::fmt;
use std::collections::btree_map::Values;

use rand::random;

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new() -> Box<Self> {
        Box::new(Self {
            value: 0,
            next: None,
        })
    }

    fn new_valued(value: i32) -> Box<Self> {
        Box::new(Self {
            value: value,
            next: None,
        })
    }

    fn new_option_valued(value: i32) -> Option<Box<Self>> {
        Some(Self::new_valued(value))
    }
}

struct Stack {
    top: Option<Box<Node>>,
    size: u32,
}

impl Stack {
    fn new() -> Self {
        Stack { top: None, size: 0 }
    }
    fn add(&mut self, value: i32) -> u32 {
        self.size += 1;
        if (self.top.is_none()) {
            self.top = Node::new_option_valued(value);
            return self.size;
        }
        let mut temp: Box<Node> = Node::new_valued(value);
        temp.next = self.top.take();
        self.top = Some(temp);
        self.size
    }
    ///  Removes a `Node` in Stack by value and return the value removed or `None`
    ///
    fn remove(&mut self, value: i32) -> bool {
        if (self.top.is_none()) {
            return false;
        }
        if let Some(ref mut data) = self.top {
            if data.value == value {
                self.size -=1;
                self.top = data.next.take();
                return true;
            }
        }
        let mut current = &mut self.top;
        while let Some(node) = current {
            if let Some(ref mut next_node) = node.next {
                if next_node.value == value {
                    self.size -= 1;
                    node.next = next_node.next.take();
                    return true;
                }
            }
            current = &mut node.next;
        }
        false
    }

    fn print_self(&self) {
        let mut temp = &self.top;
        println!("======== STACK =======");
        while let Some(data) = temp {
            println!("{}", data.value);
            temp = &data.next;
        }
    }
}

fn main() {
    let mut stack = Stack::new();

    for i in 0..10 {
        stack.add(i);
    }
    stack.print_self();
    println!("------ Remover ------");
    for i in 5..15{
        println!("Result of {i} removed is : {:?} and size {}",stack.remove(i),stack.size);
        stack.print_self();
    }
}
