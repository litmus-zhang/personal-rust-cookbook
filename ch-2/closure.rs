fn main() {
    let sum_num = |x: i32, y: i32| x + y;
    println!("7 + 8 = {}", sum_num(7, 8));

    let num_ten =  10;
    let add_ten = |x:i32| x+num_ten;
    println!("3 + 10 = {}", add_ten(3) );
}
