fn main(){

    let x_val = 5u32;
    let y_val = {
        let x_squared = x_val * x_val;
        let x_cub = x_squared * x_val;
        x_cub + x_squared +x_val
    };

    let z_val = {
        2 * x_val
    };
    println!("x is {:?}", x_val);
    println!("y is {:?}", y_val );
    println!("z is {:?}",  z_val);
}
