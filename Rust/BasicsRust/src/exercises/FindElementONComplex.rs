#[derive(Clone)]
struct Element {
    value: i32,
    indice: usize,
}

fn get_max(array: &Vec<i32>) -> i32 {
    let mut max = array[0];
    for &iterator in array.iter() { 
        if iterator > max {
            max = iterator;
        }
    }
    max
}

fn hash(value: i32, key: i32) -> usize { 
    (value % key) as usize
}

fn main() {
    let array = vec![2, 7, 7, 2, 3, 2, 7];
    let max = get_max(&array) + 1; 

    let mut array_hash: Vec<Element> = vec![Element { value: 0, indice: 0 }; max as usize]; 

    for &value in array.iter() { // Corrigi para iterar sobre referências ao invés de valores
        let key = hash(value, max);
        let element = array_hash.get_mut(key as usize); // Alterei para obter uma referência mutável
        match element {
            Some(e) => {
                e.indice += 1;
            },
            None => {
                println!("Error");
            }
        }
    }

    // Exibindo o resultado
    for (i, element) in array_hash.iter().enumerate() {
        println!("Elemento {} apareceu {} vezes", i, element.indice);
    }
}
