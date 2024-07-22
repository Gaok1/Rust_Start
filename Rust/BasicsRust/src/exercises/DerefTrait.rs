use std::ops::Deref;

struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MyBox<String> {
// impl for String variant of MyBox
    fn as_str(&self) -> &str {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(String::from("Rust"));
    //neste caso, &MyBox<String> é convertido para &String
    //seria uma coerção implicita de tipo
    hello(&y);
    //já aqui estou enviando uma referência para a string (&string)
    //e o rust converte para &str de forma implicita

    let box_explicit: &MyBox<String> = &y;
    let string_implicit:&String = &y;
    let string_slice_implicit: &str = &y;

    let box_ok: MyBox<String> = MyBox::new(String::from("Rust"));

    let box_2: &String = &(*box_ok);
    let box_3: &str = &(*box_2)[..];
    let box_3_2: &str = &(*(&(*box_ok)))[..];

}
