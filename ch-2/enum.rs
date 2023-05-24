enum Hero {
    Fast,
    Strong(i32),
    Info { name: String, secret: String },
}

fn main() {
    let hulk = Hero::Strong(100);
    let fasty = Hero::Fast;
    let spiderman = Hero::Info {
        name: "Spiderman".to_owned(),
        secret: "peter parker".to_owned(),
    };
    get_info(spiderman);
    get_info(hulk);
    get_info(fasty);
}

fn get_info(h: Hero) {
    match h {
        Hero::Fast => println!("Fast"),
        Hero::Strong(i) => println!("Lifts {} tons ", i),
        Hero::Info { name, secret } => {
            println!("name is {0}, secret is {1}", name, secret);
        }
    }
}
