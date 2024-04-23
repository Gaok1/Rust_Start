use core::panic;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::result;

// error handling
fn main() {
    panicHandler();
    callDivide(1,0);
    expectDivide(1,0);
}

fn panicHandler() {
    let greeting_file_result: Result<File, Error> = File::open("hello.txt");
    //define que greeting_file é um Result<File, Error>, pode ser um File ou pode dar um Erro

    let greeting_file = match greeting_file_result {
        //match para saber qual foi o resultado da open
        Ok(file) => file, //if its Ok unwrap the file value
        Err(error) => match error.kind() {
            // is Error, then lets work on it
            ErrorKind::NotFound => match File::create("hello.txt") {
                //if its not found, create the file
                Ok(fc) => fc, //if created ands its ok, unwrap the file
                Err(e) => panic!("Problem creating the file: {:?}", e), //if not, panic!
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error); //if error not found, panic!
            }
        },
    };
}


fn callDivide(x: i32, y: i32) -> Option<i32>{ //tratamos o erro
    
    let retorno = match divide(x,y) {
        Ok(value) => return Some(value),
        Err(e) => {

            let error: Option<ErrorKind> = match e {
                ErrorKind::InvalidInput => {
                    println!("Invalid input"); None}
                _ => { println!("Another error"); None}
            };
        }
    };
    None
}

fn interrogationDivide(x:i32, y:i32)-> Result<i32, ErrorKind>{
    /* he ? placed after a Result value is defined to work in almost the same way as the match expressions we defined
    to handle the Result values in Listing 9-6. If the value of the Result is an Ok,
     the value inside the Ok will get returned from this expression, and the program will continue. 
     If the value is an Err, the Err will be returned from the whole function */
    let result = divide(x,y)?; //literalmente um throw exceptions
    Ok(result) //retorna se tudo der certo
}

fn expectDivide(x: i32, y: i32) -> i32{ //jeito mais simples de tratar erros,mas não trata o erro de fato
    //ta mais para um throw exception
    let result = divide(x,y).expect("Failed to divide");
    result
}





fn divide(x: i32, y: i32) -> Result<i32, ErrorKind> {
    if y == 0 {
        return Err(ErrorKind::InvalidInput)
    }
    if (y == 666){
        return Err(ErrorKind::Other)
    }
    Ok(x / y)
}
