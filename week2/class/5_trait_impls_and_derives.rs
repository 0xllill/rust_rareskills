use std::fmt::{Display, Debug};

/** TRAITS
 * Allow us to define common behaviour on multiple types
 *  - Many useful ones already in the prelude and in the standard library 
 *  - Define our own with `trait`
 */
struct User {
    name: String,
    age: u8,
    additional_info: Option<Vec<String>>,
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User").field("name", &self.name).field("age", &self.age).field("additional_info", &self.additional_info).finish()
    }
}

impl User {
    fn display_data(&self) {
        println!("The user is {} and is {} years old.", self.name, self.age); 
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "The user is {} and is {} years old.", self.name, self.age)
    }
}

fn main() {
    let alice = User {
        name: String::from("Alice"),
        age: 30,
        additional_info: Some(vec![String::from("work: Security Researcher"), String::from("skills: Rust")]),
    };

    // User custom function
    alice.display_data();

    println!("Display: {}", alice);

    println!("Debug: {:?}", alice);
}