fn call_once<F>(func: F) 
where
    F: FnOnce() -> String,
{
    println!("{}", func());
    println!("{}", func());
}

fn main() {
    let s = String::from("hello");
    call_once(|| s); // move `s` para dentro da closure
    // `s` não pode ser usado aqui porque foi movido
}
