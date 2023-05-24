fn main() {
    let a = 5;
    let (b, c) = (1, 2);
    let x_val: i32 = 5;
    let y_val: i32 = 9;
    {
        println!("Value assigned when entereing the scope : {}", y_val);
        let y_val = 12;

        println!("Value modified withing scope {}", y_val);
    }
    println!("Initial value {}", y_val);
    let y_val = 32;
    println!("Final value {}", y_val);
}
