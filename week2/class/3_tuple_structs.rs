/** TUPLE STRUCTS
 * Just like the other structs, but fields are not named and accessed by index
 */
struct Wrapper(User);

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

    let wrap: Wrapper = Wrapper(alice);
    println!("{}", wrap.0.name)
}