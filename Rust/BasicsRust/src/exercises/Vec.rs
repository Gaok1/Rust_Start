use core::slice;
use std::error;


fn main(){
    let mut my_array: [i32; 10] = [1,2,3,4,5,6,7,8,9,10]; //stack allocated, not mutable size

    let mut vector:Vec<i32> = Vec::new(); //heap allocated, mutable size]

    vector.push(2); vector.push(12);
    vector.push(3); vector.push(4); 
    vector.push(5); vector.push(6);
    vector.push(7); vector.push(8);
    {
        let  third: Option<&i32> = vector.get(2);
        //third = Some(5); //third cannot be assigned a value, because it is a reference
        let  fourth: i32 = vector[3]; //not safe, can cause panic

        let  fourth: Option<&i32> = vector.get(3); //safe, returns an Option<&i32>
        
        if let Some(fourth) = vector.get(3){
            println!("O quarto elemento é {}", fourth);
        }else{
            println!("O quarto elemento não existe"); 
        }
        print!("fourth é {}\n\n=====", fourth.unwrap_or(&0));


        let does_not_exist: Option<&i32> = vector.get(100); //return None ("safe")
        // let does_not_exist: i32 = vector[100]; //panics
    }

    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let mut first: &mut i32 = &mut v[0]; //imutable borrow, cannot push // slice is alaways a reference(imutable)?
    // v.push(6); cannot edit v, because it may cause a reallocation, and the reference (first) will be invalid

    println!("The first element is: {first}");

    for i in &mut vector{
        println!("{}", (*i + 50));
    }
    

    //strings

    let mut data = "initial contents"; //stack alocated
    let mut dataHeap = "another string".to_string(); //heap alocated
    let mut dataHeap2 = String::from("another string");//same as above

    //data[0]; //cannot access a string by index, because it is a vector of bytes,
    // and the index may not be a valid char boundary

    //all Strings are UTF-8 encoded, so they can contain any valid char

    for chars in data.chars(){ // iterate all utf-8 chars
        println!("{}", chars);
    }

    for bytes in data.bytes(){ //iterate all bytes
        println!("{}", bytes);
    }

    let concat = format!("{}-{}-{}", data, dataHeap, dataHeap2); //concatenate strings

    

}