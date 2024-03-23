#[derive(Debug)]

struct Node {
    value: i32,
    next : Option<Box<Node>>, //Box é um tipo ponteiro
    //Option é um tipo enum que pode ser None ou Some (seria recursivo)
    //então next pode ser None ou Some(Box<Node>)
    //os valores de Box<Node> são alocados no heap
}
impl Node{

    fn get_value(&self) -> i32 {
        self.value
    }

    fn set_value(&mut self, value:i32){
        self.value = value;
    }

    fn get_next_slice(&self) -> &Option<Box<Node>> { 
        &self.next
    }

    fn get_next(&self) -> Option<Box<Node>> { //
        self.next;
    }

    fn set_next(&mut self, next : Option<Box<Node>>){
        self.next = next;
    }
}

#[derive(Debug)]
struct Stack{
    top: Option<Box<Node>>,
    size: usize,
}

impl Stack{

    fn create_node(value: i32) -> Node{
        Node{value,next:None}
    }

    fn get_top_slice(&self) -> &Option<Box<Node>>{
        &self.top
    }
    /// cuidado ao utilizar essa função, pois ela consome a ownership do top
    fn get_top(self) -> Option<Box<Node>>{
        self.top
    }

    fn set_top(&mut self, top:Option<Box<Node>>){
        self.top = top;
    }

    fn get_size(&self) -> usize{
        self.size
    }

    fn get_top_value(&self) -> Option<i32>{
        match &self.get_top_slice(){
            None => None,
            Some(node) => Some(node.get_value()),
        }
    }

    /// Takes Ownership of the Stack
    fn push(self, value: i32)-> Stack{
        let mut new_node: Node = Node{value,next:None};
        let tmp_top = self.get_top(); 
        new_node.set_next(tmp_top);
        return self
    }

    fn pop(self) -> Stack{
        let tmp_top = self.get_top();
        match tmp_top{
            None => return self,
            Some(node) => {
                self.set_top(node.get_next());
                return self
            }
        }
    }

} 



fn main(){
    

}