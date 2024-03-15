#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32{ //(&) ->usa referencia para não pegar ownership
        2 * (self.width + self.height)
    }

}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.\n
         And its perimeter is {} pixels.",
        rect1.area(), rect1.perimeter()
    );
}