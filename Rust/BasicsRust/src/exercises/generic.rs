// generic types serve as a way to define a function or struct that can work with any type

use std::{
    fmt::{Display, Error},
    io::IntoInnerError,
    os::windows::io::NullHandleError,
};

fn biggest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    // T is a generic type parameter
    // the function takes a slice of T and returns a reference to a T element
    // std::cmp::PartialOrd is a trait that allows us to compare values of type T, in the function its act as a restriction
    let mut biggest = &list[0];
    for i in list {
        if i > biggest {
            biggest = &i;
        }
    }
    &biggest
}

#[derive(Debug)]
struct Point<T> {
    //Point is a generic struct that takes two types T and U
    x: T,
    y: T,
}
impl<T: Display> Point<T> {
    fn to_string(&self) -> String {
        format!("x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
enum Shulambs<T, U> {
    //Shulambs is a generic enum that takes two types Ok and Err
    Ok(T),
    Err(U),
}


fn divide(x: i32, y: i32) -> Shulambs<i32, Error> {
    //Shulambs is a generic enum that takes two types i32 and Error
    use Shulambs::{Err, Ok}; //use Shulambs::{Ok,Err} is a way to bring the enum variants into scope
    if y == 0 {
        Err(Error)
    } else {
        Ok(x / y)
    }
}

fn main() -> () {
    let char_list = ['a', 'b', 'c', 'd', 'e'];
    let integer_list = [1, 2, 3, 4, 5];
    let float_list = [1.1, 2.2, 3.3, 4.4, 5.5];
    let point = Point { x: 1, y: 2 };

    println!("The biggest char is {}", biggest(&char_list));
    println!("The biggest integer is {}", biggest(&integer_list));
    println!("The biggest float is {}", biggest(&float_list));
    println!("{:?}\n", divide(1, 0));
    println!("point : {:?}", point);
    
}
