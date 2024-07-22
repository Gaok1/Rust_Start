use std::{collections::btree_map::Iter, string, vec};

use rand::distributions::weighted;

struct Cliente{
    altura : f32,
    peso : f32,
}
impl Cliente {

    fn calculo_imc(&self) -> f32{
        let imc = self.peso / (self.altura * self.altura);
        return imc;
    }

    fn filter_weight(weight:(f32, f32), clientes: Vec<Cliente>) -> Vec<Cliente> {
        clientes.into_iter().filter(|cliente|{
            let imc = cliente.calculo_imc();
            imc >= weight.0 && imc <= weight.1 //expressão booleana
        }).collect()
    }
}





fn main() {

    let mut clientes: Vec<Cliente>  = generate_random_clients();
    const normal_weight : (f32, f32) = (18.5, 24.9);

   
    printe_Imc(&clientes);
    clientes = Cliente::filter_weight(normal_weight, clientes);
    println!("\n\n|Clientes depois do método filter|\n");
    printe_Imc(&clientes);
    

    

}

fn printe_Imc(clientes: &Vec<Cliente>){
    for cliente in clientes{
        print!("IMC: {:.2} ->", cliente.calculo_imc());
        if(cliente.calculo_imc() < 18.5){
            println!("Abaixo do peso");
        }else if(cliente.calculo_imc() < 24.9){
            println!("Peso normal");
        }else if(cliente.calculo_imc() < 29.9){
            println!("Sobrepeso");
        }else if(cliente.calculo_imc() < 34.9){	
            println!("Obesidade grau 1");
        }else if(cliente.calculo_imc() < 39.9){
            println!("Obesidade grau 2");
        }else{
            println!("Obesidade grau 3");
        }

    }
}

fn generate_random_clients() -> Vec<Cliente> {
    let mut clientes : Vec<Cliente> = Vec::new();
    for _ in 0..10 {
        let altura = rand::random::<f32>() * 0.5 + 1.5;
        let peso = rand::random::<f32>() * 50.0 + 50.0;
        clientes.push(Cliente{altura, peso});
    }
    return clientes;
}