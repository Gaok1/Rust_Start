fn call_mut<F>(mut func: F) 
where
    F: FnMut(),
{
    func();
    func();
}

fn main() {
    let mut count = 0;
    call_mut(|| { //pode ser chamada mais de uma vez
        //borrow mutavel
        count += 1;
        println!("Count: {}", count);
    });
}
