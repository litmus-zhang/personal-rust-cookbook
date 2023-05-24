fn main(){
    let mut circle = Circle{
        x:10.0,
        radius: 10.0
    };

    println!("x:{}, radius:{}", circle.x, circle.radius );
    println!("x: {}",  circle.get_x());
}

struct Circle{
    x : f64,
    radius : f64,
}

impl Circle {
    pub fn get_x(&self) -> f64{
      self.x
    }
}