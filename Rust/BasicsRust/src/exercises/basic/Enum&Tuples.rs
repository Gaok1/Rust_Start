fn get_tuple_3(element_1: i32, element_2: i32, element_3: i32) -> (i32, i32, i32) {
    return (element_1, element_2, element_3);
}

fn print_tuple_3() {
    let tuple = get_tuple_3(6, 3, 10);
    for i in 0..3 {
        //não é possivel iterar tuplas, então tem essa gambiarra
        match i {
            0 => print!("Elemento 1: {}\n", tuple.0),
            1 => print!("Elemento 2: {}\n", tuple.1),
            2 => print!("Elemento 3: {}\n", tuple.2),
            _ => print!("Erro\n"),
        }
    }
}

enum WeekDays {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo,
}

fn try_enum() {
    
    for i in 0..15 {
        let hoje:WeekDays = WeekDays::Segunda;
        let dia: WeekDays = match (hoje as u8 + i) % 7 {
            0 => WeekDays::Segunda,
            1 => WeekDays::Terca,
            2 => WeekDays::Quarta,
            3 => WeekDays::Quinta,
            4 => WeekDays::Sexta,
            5 => WeekDays::Sabado,
            6 => WeekDays::Domingo,
            _ => panic!("Erro"),
        };

        match dia {
            WeekDays::Segunda => println!("{i} = Segunda"),
            WeekDays::Terca => println!("{i} = Terça"),
            WeekDays::Quarta => println!("{i} = Quarta"),
            WeekDays::Quinta => println!("{i} = Quinta"),
            WeekDays::Sexta => println!("{i} = Sexta"),
            WeekDays::Sabado => println!("{i} = Sábado"),
            WeekDays::Domingo => println!("{i} = Domingo"),
        }
    }
}

fn main() {
    print_tuple_3();
    try_enum();
}
