use std::{fmt::{self, Debug}, ops::{Shl, Shr}};

use rand::Rng;
extern crate rand;


/* Traits are a way to define shared behavior/methods between different types. */
#[derive(Debug)]
struct Circle {
    radius: f64
}
struct Square{
    side: f64
}
impl Square {
    fn new(side: f64) -> Square {
        Square{side}
    }
}
impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        self.side == other.side
    }
}

impl PartialOrd for Square {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.side.partial_cmp(&other.side)
    }
}

impl fmt::Display for Square{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Square with side: {:.1} and area: {:.2}", self.side, self.area())
    }
}

impl Shl<i32> for Square{
    type Output = Square;
    ///Operator overload to sum value into the square side
    /// 
    ///# Example
    /// 
    /// square << 1  `is the same` square.side +=1
    fn shl(self, rhs: i32) -> Square {
        
        return Square::new(self.side + rhs as f64);
    }
}

impl Shr<i32> for Square{
    type Output = Square;
    ///Operator overload to subtract value into the square side
    /// 
    ///# Example
    /// 
    /// square >> 1  `is the same` square.side -=1
    fn shr(self, rhs:i32) -> Square {
        
        return Square::new(self.side - rhs as f64);
    }
}

struct Triangle{
   pub base: f64,
   pub height: f64 //pq não pode ser private? -> pq a função area() precisa acessar esses valores
}
pub trait Geomtry {
    fn area(&self) -> f64;
    
    fn print_form(&self) -> String {
        String::from("Forma Geométrica desconhecida ou não implementada")
    }
    fn volume(&self, height: f64) -> f64 {
        height*0.0
    }
}

impl Geomtry for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn print_form(&self) -> String {
        String::from("Círculo")
    }
    fn volume(&self, height: f64) -> f64 {
        self.area() * height
    }
}

impl Geomtry for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn volume(&self, height: f64) -> f64 {
        self.area() * height
    }
}

impl Geomtry for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
    fn print_form(&self) -> String {
        String::from("Triângulo")
    }
    fn volume(&self, height: f64) -> f64 {
        self.area() * height
    }
}

fn half_volume(g: &impl Geomtry, height : f64) -> f64 {
    if(height != 0.0){
        return g.volume(height) / 2.0 
    }
    return 0.0
}
fn half_area( g : &impl Geomtry) -> f64 {
     g.area() / 2.0
}
//with trait bounds -> generic function
//same as above but with trait bounds
pub fn half_volume_2<T: Geomtry>(form : &T, height : f64)-> f64 {
    if(height != 0.0){
        return form.volume(height) / 2.0
    }
    return 0.0
}
//we can add more trait bounds with + operator
pub fn diplay_half_area_2<T: Geomtry + Debug>(form : &T) {
    println!("Forma: {:?} e sua area metade: {:.2}", form, half_area(form));
}
//where clause
pub fn diplay_half_area_3<T>(form : &T) 
where T: Geomtry + Debug  //t must implements Geomtry and Debug
{
    println!("Forma: {:?} e sua area metade: {:.2}", form, half_area(form));
}




fn main(){
    let c = Circle{radius: 5.0};
    let s = Square{side: 5.0};
    let t = Triangle{base: 5.0, height: 5.0};

    println!("Area of Circle: {:.2}", c.area());
    println!("Area of Square: {:.2}", s.area());
    println!("Area of Triangle: {:.2}", t.area());

    println!("Forma do círculo: {}", c.print_form());
    println!("Forma do quadrado: {}", s.print_form());
    println!("Forma do triângulo: {}", t.print_form());

    println!("volume of Circle: {:.2}", c.volume(10.0));
    println!("volume of Square: {:.2}", s.volume(10.0));
    println!("volume of Triangle: {:.2}", t.volume(10.0));

    println!("Half volume of Circle: {:.2}", half_volume(&c,10.0));
    println!("Half volume of Square: {:.2}", half_volume(&s,10.0));
    println!("Half volume of Triangle: {:.2}", half_volume(&t,10.0));

    diplay_half_area_2(&c);

    
}