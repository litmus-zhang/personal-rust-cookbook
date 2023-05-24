fn main() {
    let age = 10;
    if age <= 18 {
        println!("Go to school");
    } else if (age > 18) && (age <= 20) {
        println!("Go to college");
    } else {
        println!("Do something with your life");
    }

    let can_vote = if age >= 18 {true} else { false};

    println!("Can vote {}",can_vote );
}
