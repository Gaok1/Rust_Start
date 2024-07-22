use rand::Rng;

#[derive(Debug, PartialEq)]
enum Diamonds {
    Red,
    Blue,
}

struct Mine {
    dwarfs: Vec<Diamonds>,
}

impl Mine {
    fn manage_dwarf(&mut self, dwarf: Option<Diamonds>) -> Diamonds {
        let red_weight = 0.75;
        let blue_weight = 0.25;
        dwarf.unwrap_or_else(|| self.get_needing(red_weight, blue_weight))
        //Closure captures the red_weight, blue_weight and self variables
    }

    fn get_needing(&self, red_weight: f64, blue_weight: f64) -> Diamonds {
        let mut blue: f64 = 0.0;
        let mut red: f64 = 0.0;
        for i in self.dwarfs.iter() {
            match i {
                Diamonds::Red => red += 1.0,
                Diamonds::Blue => blue += 1.0,
            }
        }
        red = red * red_weight;
        blue = blue * blue_weight;
        println!("Red: {}, Blue: {}", red, blue);
        if red > blue {
            Diamonds::Blue
        } else {
            Diamonds::Red
        }
    }
}

fn main() {
    let mut mine = Mine {
        dwarfs: vec![Diamonds::Red, Diamonds::Red, Diamonds::Blue, Diamonds::Blue],
    };

    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let preference: Option<Diamonds> = if rng.gen_bool(0.7) {
            Some(Diamonds::Red)
        } else {
            Some(Diamonds::Blue)
        };

       
    
        print!("Preference: {:?}, ", preference);
        let result: Diamonds = mine.manage_dwarf(preference);
        println!("Result: {:?}", result);
    }

    // Testando anões sem preferência específica
    for _ in 0..10 {
        let result = mine.manage_dwarf(None);
        println!("No preference, Result: {:?}", result);
        mine.dwarfs.push(result);
    }
}
