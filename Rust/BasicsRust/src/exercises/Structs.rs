

struct User {
    id: i32,
    active: bool,
    name: String,
    email: String,
}

struct Color(i32, i32, i32); // apesar de seren tuplas de mesmo tamanho e tipo
struct Origin(i32, i32, i32);// não são do mesmo tipo, então não podem ser atribuidas uma a outra
fn testTuple(){
    let black = Color(0, 0, 0);
    let origin = Origin(0, 0, 0);
    black = origin; //Erro, origin não é do mesmo tipo que black
    origin = black; //Erro, black não é do mesmo tipo que origin
    let Color(r, g, b) = black; //desestruturação de tupla
    let Origin(x, y, z) = origin; //desestruturação de tupla
    println!("r: {}, g: {}, b: {}", r, g, b);
    println!("x: {}, y: {}, z: {}", x, y, z);
}

fn main() {
    let mut tuples: (i32, i32, i32) = (1, 2, 3);

    let mut usuario: User = User {
        id: 1,
        active: true,
        name: String::from("Joao Bastos"),
        email: String::from("blablabla@gmail.com"),
    };

  let user2 = User {
      email: String::from("another@example.com"),
      ..usuario //atribuindo os valores de usuario para user2
      //'=' sinaliza movimentação de dados, então pode ocorrer copy de variaveis ou
      //borrowing de variaveis
  };

  while tuples.0 < 10 {
      println!("tuples.0: {}", tuples.0);
      tuples.0 += 1;
  }

  //usuario.set_name(String::from("Joao Bastos")); //Erro, usuario foi movido para user2[
  //valor name não implementa copy trate, então ocorrerá movimentação de ownership
  

  
  
  
  
    
}

fn build_user(id:i32,active:bool, name: String,email: String) -> User {
    User {
        id,
        active,
        name,
        email,
    }
}

fn build_empty_user() -> User { 
    User {
        id: 0,
        active: false,
        name: String::from(""),
        email: String::from(""),
    }
}

impl User {

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_email(&self) -> &String {
        &self.email
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_email(&mut self, email: String) {
        self.email = email;
    }

    fn is_active(&self) -> bool {
        self.active
    }
    fn to_string(&self) -> String {
        format!("id: {}, name: {}, email: {}", self.id, self.name, self.email)
    }
    
}

