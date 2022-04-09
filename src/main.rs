use std::io::stdin;

#[derive(Debug)]
struct Guest {
    name: String,
    greeting: String
}

impl Guest {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string()
        }
    }


    fn greet(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut name = String::new();

    stdin().read_line(&mut name).expect("Failed to read line");

    name.trim().to_lowercase()
}

fn main() {
    let mut guest_list = vec![
        Guest::new("max", "Hello Max, enjoy your treehouse."),
        Guest::new("lucile", "Hi Lucile, your coke is in the fridge."),
        Guest::new("angus", "Hey little Angus, always smiling I see.")
    ];
    loop {
    println!("Hello, what's your name?");

    let name = what_is_your_name();

    let known_guest = &guest_list.iter().find(|g| g.name == name);

    match known_guest {
        Some(guest) => guest.greet(),
        None => {
            if name.is_empty() {
                break;
            } else {
                println!("{} is not on the guest list", name);
                guest_list.push(Guest::new(&name, "New friend"));
            }
        },
    }
    }

    println!("Final list of guests:");
    println!("{:#?}", guest_list);
}
