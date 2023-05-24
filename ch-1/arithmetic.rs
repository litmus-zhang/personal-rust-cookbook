fn main() {
    let (addition, subtraction, multiplication, division) = (5 + 4, 5 - 4, 5 * 4, 5 / 4);
    println!("Addition = {}", addition);
    println!("Subtraction = {}", subtraction);

    println!("Multiply = {}", multiplication);
    println!("Division = {}", division);
    let neg_4 = -4i32;
    println!("abs(-4)= {}", neg_4.abs());
    println!("power(-4**2) = {}", neg_4.pow(2));
    println!("round(1.23456)={}", 1.2345f64.round());
    println!("ceil(1.2345) = {}", 1.2345f64.ceil());
    println!("sin 3.14 = {}", 3.14f64.sin());
}
