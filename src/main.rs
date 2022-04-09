use std::io::stdin;

#[derive(Debug)]
enum GuestAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Guest {
    name: String,
    action: GuestAction,
    age: i8,
}

impl Guest {
    fn new(name: &str, action: GuestAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet(&self) {
        match &self.action {
            GuestAction::Accept => println!("Welcome to the treehouse, {}", capitalize(&self.name)),
            GuestAction::AcceptWithNote { note } => {
                println!(
                    "Welcome to the treehouse, {} ({})",
                    capitalize(&self.name),
                    note
                );
                if self.age < 18 {
                    println!("No alcohol!");
                }
            }
            GuestAction::Probation => {
                println!("{} is now a probationary member", capitalize(&self.name))
            }
            GuestAction::Refuse => println!("Do not allow {} in!", capitalize(&self.name)),
        }
    }
}

fn ask_name() -> String {
    let mut name = String::new();

    stdin().read_line(&mut name).expect("Failed to read line");

    name.trim().to_lowercase()
}

fn ask_age() -> i8 {
    let mut age = String::new();
    stdin().read_line(&mut age).expect("Failed to read line");

    age.trim().parse::<i8>().expect("Failed to parse string")
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() {
    let mut guest_list = vec![
        Guest::new("max", GuestAction::Accept, 31),
        Guest::new(
            "lucile",
            GuestAction::AcceptWithNote {
                note: "Hide chocolate".to_string(),
            },
            35,
        ),
        Guest::new("angus", GuestAction::Accept, 0),
        Guest::new("justine", GuestAction::Refuse, 29),
    ];
    loop {
        println!("Hello, what's your name?");

        let name = ask_name();

        let known_guest = &guest_list.iter().find(|g| g.name == name);

        match known_guest {
            Some(guest) => guest.greet(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("Hmm, you are not on the guest list...");
                    println!("How old are you?");

                    let age = ask_age();
                    let new_guest = Guest::new(&name, GuestAction::Probation, age);
                    new_guest.greet();
                    guest_list.push(new_guest);
                }
            }
        }
    }

    println!("Final list of guests:");
    println!("{:#?}", guest_list);
}
