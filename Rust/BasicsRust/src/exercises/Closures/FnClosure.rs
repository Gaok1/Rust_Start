fn call_fn<F>(func: F)
where
    F: Fn(i32) -> i32,
{
    println!("Result: {}", func(3));
    println!("Result: {}", func(3));
}

fn main() {
    let weight = 2;
    let multiply = |x: i32| x * weight; //borrow imutavel
    //obedece a Fn trait
    call_fn(multiply);
    call_fn(multiply);
}


