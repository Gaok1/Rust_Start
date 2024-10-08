// ==== Strings ====

fn get_string_size(s: &String) -> usize{
    s.len()
}

fn get_string_index(s: &String, index: usize) -> char{
   let buf = s.chars().nth(index).unwrap();
   return buf;
}

fn copy_string(s: &str) -> String{ //apenas ler a string
    let mut new_string = String::new();
    new_string.push_str(s);
    new_string.char_indices();
    return new_string;
}



fn main(){
    let mut my_string:String = String::from("Hello,World!");
    //String mutavel e dinamica
    my_string  += "My name is top top";

    let mut other_string:&str = "Hello,World!";
    //String imutavel e estatica
    //other_string += 'a'; //erro pois a string é imutavel (slice de leitura)]
   

    print!("{}\n", my_string); print!("{}\n", other_string);

    print!("Tamanho da string: {}\n", get_string_size(&my_string));
    print!("Caracter na posição 0: {}\n", get_string_index(&my_string, 0));
    print!("Tamanho da String other_string: {}\n", other_string.len());
    print!("Caracter na posição 10: {}\n", other_string.chars().nth(10).unwrap());


    print!("Copia da string: {}\n", copy_string(&my_string));


}