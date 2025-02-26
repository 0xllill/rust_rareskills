use std::fmt::Display;

struct User {
    name: String,
    age: u8,
    action: String,
    confidential_info: Option<Vec<String>>,
}

type Thisy = dyn Action;

/** TRAIT DEFN
 * Can have default behaviour in the definition, and can overwrite it in the impl
 */
trait Action {
    // fn something() {
    //     println!("Something")
    // }

    fn action(&self) -> String {
        String::from("ABCDEFG")
    }
}

impl User {
    fn something() {
        println!("Something")
    }
}

impl Action for User {
    // fn action(&self) -> String {
    //     let mut v = format!("What does {} do for a living? ", self.name);
    //     v.push_str(&format!("{} {}.", self.name, self.action));
    //     v
        
    // }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.confidential_info {
            Some(_) => write!(f, "{} is {} years old, {} and has confidential data.", self.name, self.age, self.action),
            None => write!(f, "{} is {} years old and {}.",  self.name, self.age, self.action),
        }
    }
}

fn main() {
    let mut alice = generate_user();

    let alice_action = alice.action();
    println!("{}", alice_action);
    println!("{}", alice);

    alice.confidential_info = Some(vec![String::from("work: SR"), String::from("skills: Rust")]);
    println!("Output should change: {}", alice);

    // User::something();
}









// Appendix

fn generate_user() -> User {
    User {
        name: String::from("Alice"),
        age: 30,
        action: String::from("breaks code"),
        confidential_info: None,
    }
}