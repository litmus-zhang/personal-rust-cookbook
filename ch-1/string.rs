fn main() {
    let rand_string = "I love rust alot <3";
    println!("Length of string is = {}", rand_string.len());

    let (first, second) = rand_string.split_at(7);
    println!("First: {0}, Second: {1}", first, second);

    let count = rand_string.chars().count();
    println!("Count {}", count);

    println!("------------------------");
    let mut chars = rand_string.chars();
    let mut indiv_chars = chars.next();
    loop {
        match indiv_chars {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_chars = chars.next();
    }
}
