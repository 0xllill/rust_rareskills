/** STRUCTS
 * Structs store hetrogenous data, and the fields are named
 * We access the fields with Struct.field_name notation
 * The data that can be store in a struct must be statically known size. Except the last position can be a DST
 *  - Dynamically Sized Type, unknown size at compile time
 *  - This is because there needs to be predictability with layout
 */
struct User {
    name: String,
    age: &'static [u8],
    additional_info: Vec<u8>,
}


fn main() {
    // let mut alice = User {
    //     name: String::from("Alice"),
    //     age: 30,
    //     additional_info: None,
    // };
    // println!("The user is {} and is {} years old.", alice.name, alice.age);
    // match alice.additional_info {
    //     Some(infos) => println!("Additional info [ {} ]", infos.join(",")),
    //     None => println!("No additional info for user"),        
    // }

    // alice.additional_info = Some(vec![String::from("work: Security Researcher"), String::from("skills: Rust")]);

    // println!("The user is {} and is {} years old.", alice.name, alice.age);
    // match alice.additional_info {
    //     Some(infos) => println!("Additional info [ {} ]", infos.join(", ")),
    //     None => println!("No additional info for user"),        
    // }

}