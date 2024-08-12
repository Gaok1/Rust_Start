use std::result;


#[derive(Debug)]
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String)
}





fn main() {


    let some_number: Option<i32> = Some(4); //Option is a enum that can be Some or None
    let absent_number: Option<i32> = None;

    println!("{:?}, {:?}", some_number, absent_number);
    //values

    let x: Option<f64> = divide(0.0, 2.0);
    println!("x equals {:?}", x);
    // x = x + 5; //Error, x is a Option, so we can't use it as a number
    let mut result: f64;
    if (x.is_some()) {
        //if there is something in x
        println!("x is a number");
        println!("x is {:?}", x);
        result = x.unwrap();
    } else {
        println!("x is not a number");
        result = 0.0;
    }

    println!("result is {}", result);

    //Match
    /* Rust has an extremely powerful control flow construct called match that allows you
    to compare a value against a series of patterns and then execute code based on which pattern matches.
    Patterns can be made up of literal values, variable names, wildcards, and many other things; */

    let mut coin: Coin = Coin::Penny;
    println!("The value of the coin is {}", value_in_cents(&coin));
    coin = Coin::Quarter(UsState::Alaska);
    println!("The value of the coin is {}", value_in_cents(&coin));
    println!("Is it a nickel? {}\n ", is_Nickel(coin));
    coin = Coin::Nickel;
    println!("Is it a nickel? {}\n ", is_Nickel(coin));

    //matching com Option <t>
    let five = Some(5);
    let null:Option<i32> = None;
    println!("Soma de {:?} com 5 resulta em: {:?}\n",five, sum(5, five));
    println!("Soma de {:?} com 5 resulta em: {:?}\n",null, sum(5, null));

    //vamos testar com um enum que contenha tuplas
    let home: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
    match home {
        IpAddrKind::V4(a, b, c, d) => println!("IPV4 equals {}:{}:{}:{}", a, b, c, d),
        IpAddrKind::V6(s) => println!("IPV6 equals {}", s),
    }

}

fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None // Não é possível dividir por zero, então retornamos None
    } else {
        Some(x / y) // Retorna o resultado da divisão
    }
}

fn sum(var:i32, x: Option<i32>)-> Option<i32>{
    match x{
        None => None, //se for do tipo None, retorna None
        Some(value) => Some(value + var), //se tiver algum valor(tipo Some)
        //extraia o valor para value e retorne o Some (tem q ser Option <>) com a soma de value + var
    }
}

fn is_Nickel(coin:Coin) -> bool{
    match coin{
        Coin::Nickel => true,
        _ => false, //default
        
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn value_in_cents(coin:& Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5, //We can execute code in the match using expressions
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
