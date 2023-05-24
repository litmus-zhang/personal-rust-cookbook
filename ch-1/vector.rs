fn main() {
    let mut vec1 = vec![1, 2, 3, 4, 5];
    println!("Item 3: {}", vec1[2]);

    for i in &vec1 {
        println!("{}", i);
    }
    vec1.push(6);
    println!("Vector after push {:?}", vec1);
    vec1.pop();
    println!("Vector after pop {:?}", vec1)
}
