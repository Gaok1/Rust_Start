




fn main(){
    let my_array: [i32; 10] = [1,2,3,4,5,6,7,8,9,10]; //stack allocated, not mutable size

    let mut vector:Vec<i32> = Vec::new(); //heap allocated, mutable size]

    vector.push(2); vector.push(12);
    vector.push(3); vector.push(4); 
    vector.push(5); vector.push(6);
    vector.push(7); vector.push(8);
    
    let mut third: Option<&i32> = vector.get(2);
    //third = Some(5); //third cannot be assigned a value, because it is a reference
    let mut fourth = vector[3]; //not safe, can cause panic

    let mut fourth = vector.get(3); //safe, returns an Option<&i32>
    
    if let Some(fourth) = vector.get(3){
        println!("O quarto elemento é {}", fourth);
    }else{
        println!("O quarto elemento não existe"); fourth = None;
    }
    print!("fourth é {}\n\n=====", fourth.unwrap_or(&0));

}