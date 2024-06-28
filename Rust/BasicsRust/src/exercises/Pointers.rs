//pointers

fn main() {
    convetionalPointer();
    heapUsage();
}



fn convetionalPointer() {
    let mut x = 5;
    
        let mut pde_x = &mut x;
        *pde_x = 6;
        *pde_x +=1;
        println!("Valor da referência pde_x: {}", pde_x);


    println!("Valor de x: {}", x); 
}

struct Node {
    value:i32,
    next: Option<Box<Node>> //ou tem um ponteiro, ou um None
}

//heap usage
fn heapUsage() {

    let s1 = String::from("hello");
    let mut dynamic_array: Vec<i32> = vec![];
    for i in 0..10 {
        dynamic_array.push(i);
    }

    let n1 = Node {
        value:1,
        next: None
    };
    
       let n2 = Node {
        value:2,
        next: Some(Box::new(n1))
    }; 
    
    
    let n3 = Node {
        value:3,
        next: Some(Box::new(n2))
    };

    fn printList(head: &Node){
        let mut current = head;
        loop {
            println!("Valor do nó: {}", current.value);
            match &current.next {
                Some(next) => current = next.as_ref(),
                None => {println!("Fim da lista"); break;}
            }
        }
    }


    printList(&n3);
    
}
