enum Something {
    A,
    B(Person),
    C {
        person: Person,
        age: u8,
    }
}

/** ENUMERATIONS
 * Complex data types for variation
 * - Variants can be Simple, or complex like structs or tuples
 * - These are commonly used to allow for control flow on types.
 *  - Easy match statements
 *  - Easy if let blocks
 * Person::User
 */
enum Person { 
    User(User),
    Admin(Administrator),
}

struct User {
    name: String,
    age: u8,
    additional_info: Option<Vec<String>>,
}

struct Administrator {
    name: String,
    identifier: [u8; 32]
}

fn generate_user() -> User {
    User {
        name: String::from("Alice"),
        age: 30,
        additional_info: None,
    }
}


fn generate_admin() -> Administrator {
    Administrator {
        name: String::from("Bob"),
        identifier: [0u8; 32],
    }
}

fn print_person(person: &Person) {
    match person {
        Person::User(user) => {
            println!("The user is {} and is {} years old.", user.name, user.age);
            match &user.additional_info {
                Some(infos) => println!("Additional info [ {} ]", infos.join(", ")),
                None => println!("No additional info for user"),        
            }
        },
        Person::Admin(admin) => {
            println!("Administrator is: {} and his ID is: {:?}", admin.name, admin.identifier);
        },
    }

}

fn main() {
    let alice = Person::User(generate_user());
    let mut admin1 = Person::Admin(generate_admin());

    print_person(&alice);
    print_person(&admin1);

    if let Person::Admin(a) = &mut admin1 {
        a.identifier = [1_u8; 32];
    }
    print_person(&admin1);
}