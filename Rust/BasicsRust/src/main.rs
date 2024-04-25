/* Traits are a way to define shared behavior/methods between different types. */

struct Circle {
    radius: f64
}
struct Square{
    side: f64
}
struct Triangle{
   pub base: f64,
   pub height: f64 //pq não pode ser private? -> pq a função area() precisa acessar esses valores
}
pub trait Area {
    fn area(&self) -> f64;
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

struct generico<Fer>{
    valor :Fer,
}


fn main(){
    let c = Circle{radius: 5.0};
    let s = Square{side: 5.0};
    let t = Triangle{base: 5.0, height: 5.0};

    println!("Area of Circle: {:.2}", c.area());
    println!("Area of Square: {:.2}", s.area());
    println!("Area of Triangle: {:.2}", t.area());
    
}