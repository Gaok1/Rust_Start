use std::ops::Add;


struct Celsius(f64);

impl Celsius {
    fn to_fahrenheit(&self) -> f64 {
        self.0 * 1.8 + 32.0
    }

}
impl Add for Celsius{
    type Output = Celsius;

    fn add(self, rhs: Celsius) -> Self::Output {
        Celsius(self.0 + rhs.0)
    }
}

impl Add<f64> for Celsius {
    type Output = Celsius;

    fn add(self, rhs: f64) -> Self::Output {
        Celsius(self.0 + rhs)
    }
}



fn main(){

    let temp = Celsius(30.0);
    println!("{}°C is {}°F", temp.0, temp.to_fahrenheit());

    let temp2 = temp + 10.0;
    println!("{}°C is {}°F", temp2.0, temp2.to_fahrenheit());


}