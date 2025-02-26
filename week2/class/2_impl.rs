/** IMPLEMENTATIONS
 * Functions that are associated with complex
 * Methods can refer to the User type in 4 ways:
 *  - Self - This refers to the datatype itself (`User`)
 *  - self - A reference to the User in memory, and this reference is `move`
 *  - &self - This is a borrow reference
 *  - &mut self - This is a mutable borrow reference
 */
impl User {
    fn pure() {
        println!("I AM PURE")
    }

    fn new(name: String, age: u8, additional_info: Option<Vec<String>>) -> Self {
        User {
            name,
            age,
            additional_info,
        }
    }

    fn moving(self) {
        println!("moving {}", self.name);
        // All owned data is dropped, moving owns self which is the User in memory - so it drops
    }

    fn borrowing(&self) {
        println!("borrowing {}", self.name);
        // All owned data is dropped, moving does not own self as it is a borrow
    }

    fn mut_borrowing(&mut self) {
        self.age += 1;
        println!("mut borrow {}, age = {}", self.name, self.age);
        // All owned data is dropped, moving does not own self as it is a borrow
    }
}

//* Structs are coarse grained with ownerships (not difference between fields for ownership) */
struct User {
    name: String,
    age: u8,
    additional_info: Option<Vec<String>>,
}


fn main() {
    let mut alice = User {
        name: String::from("Alice"),
        age: 30,
        additional_info: None,
    };
    println!("The user is {} and is {} years old.", alice.name, alice.age);

    // alice.moving();
    // alice.moving(); // <-- Can't access after a move

    alice.mut_borrowing();
    alice.mut_borrowing();

    User::pure();

    // let bob = User::new();
}