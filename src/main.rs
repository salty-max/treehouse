use std::io::stdin;

fn what_is_your_name() -> String {
    let mut name = String::new();

    stdin().read_line(&mut name).expect("Failed to read line");

    name.trim().to_lowercase()
}

fn main() {
    let guest_list = ["max", "lucile", "angus"];

    println!("Hello, what's your name?");

    let name = what_is_your_name();

    let mut allow_them_in = false;

    for guest in &guest_list {
        if guest == &name {
            allow_them_in = true
        }
    }

    if allow_them_in {
        println!("Welcome to the Treehouse, {}", &name);
    } else {
        println!("GTFO!")
    }
}
