fn main(){
    let tuple = ("Rust", "3030");
    let tuple2 : (&str, i8) = ("Valhalla", 4);

    println!("Name : {}", tuple.0);
    println!("Lucky no : {}", tuple2.1);
}