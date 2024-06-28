




fn main() {
    let string1 = String::from("abcd");
    let mut result;
    {
        let mut string2 = String::from("asdasdasdasdasd"); 
        result = logest(string2.as_str(), string1.as_str());
        //String não vive o suficiente para ser retornada
        //mas se a referencia for de uma string estatica, o lifetime vive o suficiente
        let string3: &str = "xyzasdasd";
        result = logest(string3, string1.as_str()); 
        //mesmo que string3 seja tenha escopo menor, sua referencia é de uma string estatica
    }
    println!("The longest string is {}", result);  
}
//nesse caso, a anotação de lifetime indica que o lifetime de x e y são iguais
// ou seja, o retorno da função será o mesmo lifetime de x e y
fn logest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//fn longest(x: &str, y: &str) -> &str { Erro, pois devido ao lifetime o compilador não pode escolher 
// entre x e y, pois ambos são referenciados por &str e não se sabe o lifetime individual de cada um

//if x.len() > y.len() {
//   x
// } else {
//       y
//   }
//}


