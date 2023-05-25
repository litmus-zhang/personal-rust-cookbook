extern crate regex;
use regex::Regex;

fn main() {
    let check_date = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match?  {}", check_date.is_match("2001-02-22"));
    println!("Hello, world!");
}
