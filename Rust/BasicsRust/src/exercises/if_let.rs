

fn main(){
    //if let
    let variable: Option<i32> = None;

    if let Some(value) = variable{//ao inves de testarmos a variavels para cada tipo de valor
        //testamos diretamente o valor com a variavel
        println!("value = {}", value);
    }else if let None = variable{
        println!("value = None");
    }
    /* Using if let means less typing, less indentation, and less boilerplate code */
    // utilizamos quando temos um alvo bem definido e que não queremos testar para todos os valores
    let day: week = week::Monday;

    if let week::Monday = day{
        println!("Today is Monday");
    }else{
        println!("Today is not Monday");
    }

}

enum week{
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}