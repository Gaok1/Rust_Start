//Slices

fn first_word(s: &String) -> &str {
    //retorna slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s: String = String::from("hello world");
    let mut my_slice: &str = &s[0..5];

    let y: i32 = {
        let x: i32 = 5;
        x + 1 //y recebe esse valor
    };

    // word still has the value 5 here, but there's no more string that we could meaningfully use the value 5 with. word is now totally invalid!
    println!("The first word is: {}", s);
    println!("o valor de y é: {}", y);
    for character in my_slice.chars() {
        print!("{}", character);
    }

    my_slice = first_word(&s);

    println!("\nThe first word is: {}", my_slice);

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let fe: &[i32] = &my_array[0..4];

    println!("VARIAVEL FER");
    for i in fe.iter() {
        print!("{}", i);
    }
    println!();

    let my_array_slice = get_array_slice(&my_array, 0, 3);

    for i in my_array_slice.iter() {
        print!("{}", i);
    }
}

fn get_array_slice(array: &[i32], start: usize, end: usize) -> &[i32] {
    &array[start..end]
}
