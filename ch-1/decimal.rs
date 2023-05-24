fn main(){
    println!("{:2}", 1.2345);
    println!("==============");

    println!("B: {:b} H: {:x} O: {:o}", 10,10,10);
    println!("==============");

    println!("{ten:>ws$}",ten=10, ws=5);
    println!("ten:>0ws$",ten=10, ws=5);
println!("{ten:>ws$}",ten=10, ws=5 );
println!("{ten:>0ws$}",ten=10, ws=5 );
}