#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn get_value(&self) -> i32 {
        self.value
    }

    fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    fn get_next_slice(&self) -> &Option<Box<Node>> {
        &self.next
    }

    fn get_next(&self) -> Option<&Box<Node>> {
        self.next.as_ref()
    }

    fn set_next(&mut self, next: Option<Box<Node>>) {
        self.next = next;
    }
}

#[derive(Debug)]
struct Stack {
    top: Option<Box<Node>>,
    size: usize,
}

impl Stack {
    fn new() -> Self {
        Stack { top: None, size: 0 }
    }

    fn create_node(value: i32) -> Node {
        Node { value, next: None }
    }

    fn get_top_slice(&self) -> &Option<Box<Node>> {
        &self.top
    }

    fn get_top(&self) -> Option<&Box<Node>> {
        self.top.as_ref()
    }

    fn set_top(&mut self, top: Option<Box<Node>>) {
        self.top = top;
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_top_value(&self) -> Option<i32> {
        match self.get_top() {
            None => None,
            Some(node) => Some(node.get_value()),
        }
    }

    fn push(&mut self, value: i32) {
        let mut new_node = Node::create_node(value);
        new_node.set_next(self.top.take());
        self.top = Some(Box::new(new_node));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<i32> {
        self.top.take().map(|node| {
            self.top = node.next;
            self.size -= 1;
            node.value
        })
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("{:?}", stack);

    println!("Popped value: {:?}", stack.pop());
    println!("Popped value: {:?}", stack.pop());
    println!("Popped value: {:?}", stack.pop());
    println!("Popped value: {:?}", stack.pop());

    println!("{:?}", stack);
}
