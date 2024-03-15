use core::borrow;
use std::string;

fn main() {
    //ownership
    let mut s: String = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
    println!("String after takes_ownership{}", s); //erro, s foi movido para takes_ownership

    s = String::from("hello");
    s = takes_ownership_and_gives_back(s);

    println!("String after loose ownership and get it back{}", s);
    take_borrow(&s);
    print!("String after borrow at Main{}", s); //erro, s foi movido para take_ownership_and_gives_back

    //borrowing

    let mut s = String::from("hello");

    let r1: &mut String = &mut s;
    let r2: &mut String = &mut s; //erro, s ja foi mutavelmente empresado
    println!("{}, {}", r1, r2);
    //pode-se resolver com escopo
    /*
      {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    */

    let r1: &String = &s;
    let r2: &String = &s;
    //let r3: &mut String = &mut s; //erro, s ja foi emprestado imutavelmente
    println!("{}, {},", r1, r2);

    //but if variable is not used after r1 and r2, it is possible to do this
    let r1: &String = &s;
    let r2: &String = &s;
    println!("{}, {},", r1, r2);
    let r3: &mut String = &mut s;

    //Dangling References
    let dangle_reference = dangle();

    fn dangle() -> &String {
        //this function's return type contains a borrowed value, but there is no value for it to be borrowed from
        let s = String::from("hello");
        &s
    } // s goes out of scope, and is dropped. Its memory goes away.
      // Danger!
      //Solution -> return the ownership
    fn not_dangle() -> String {
        //return ownership
        let s = String::from("hello");
        s
    }
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn takes_ownership_and_gives_back(some_string: String) -> String {
    some_string
}

fn take_borrow(borrowed_str: &String) {
    //'&' indica referenciar. então borrowed_str é uma referência para um String
    // não tem ownership
    println!("i got borrowed ->{}", borrowed_str);
}
