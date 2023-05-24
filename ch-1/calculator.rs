use std::io;
use std::{i32};


fn main(){
    //Get first input
    println!("Enter first number ? ");
    let mut input1  = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    println!("ENter second number ?");
    let mut input2 =  String::new();
    io::stdin().read_line(&mut input2).expect("Failed to readline");

    let aint: i32 = input1.trim().parse().ok().expect("Program only processes numbers, enter number");
    let bint :i32 = input2.trim().parse().ok().expect("Program only accept numbers");

    println!("Sum is : {} ", aint + bint);
    println!("Difference is  : {}", aint - bint);
    println!("Multiplication is  : {}", aint * bint);
    println!("Division is  : {}", aint / bint);
}