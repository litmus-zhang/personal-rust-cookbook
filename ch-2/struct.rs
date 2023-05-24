fn main(){
    let mut circle = Circle{
        x:10.0, 
        radius: 10.0
    };
    println!("x:{}, radius: {}",circle.x, circle.radius );
    println!("Radius : {}",  get_radius(&circle));
}

struct Circle{
    x: f64,
    radius: f64
}

fn get_radius(c1 : &Circle) -> f64{
    c1.radius
}