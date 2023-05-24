fn main(){
    let mut circle = Circle{
        r : 10.0
    };
    println!("Area of Circle is {}", circle.area() );
    let mut rect = Rectangle{
        h: 10.0,
        b: 10.0
    };
    println!("Area of rectangle {}", rect.area() );
}

struct Rectangle{
    h: f64,
    b: f64,
}

struct Circle{
    r: f64
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle{
    fn area(&self) -> f64 {
        3.14 * (self.r * self.r)
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.h * self.b
    }
}