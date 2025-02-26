fn function_1(var: u32) {

    println!("In function_1, variable is: {}", var);
}

fn main() {
    //E scalar types in Rust implement the Copy trait
    let variable: u32 = 32;

    //E original String version fails to compile because:
        // String is a more complex type that manages heap memory
        // Strings don't implement the Copy trait by default

    function_1(variable);

    println!("In main, variable is: {}", variable);
}
