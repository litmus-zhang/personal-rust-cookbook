fn main(){
    let array = [1,2,3,4,5];
    println!("Random array {:?}", array);

    println!("First Element: {},Second Element: {}", array[0], array[1]);
    println!("Length of array {}", array.len());

    println!("Last two element {:?}", &array[1..array.len()-1]);
    println!("Testing")

}