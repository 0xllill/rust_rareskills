
// 1
fn main() {
    //E modify to mut
    let a = vec![1,2,3,4];
    a.push(27);
}

// 2 

fn my_operation(a: u64, b: u64) -> u64 {
    a += b; //E parameters immutable by default => make a new variable or make parameters mutables
    a
}


fn main() {
    let num1 = 1234;
    let num2 = 1122;
    println!("My result is {}!", my_operation(num1, num2));
}


// 3

fn main() {
    let x = 1;

    {
        let mut x = 2;

        x = x ^ 2;

        {
            //E modify mutable x declared above in the inner scope so value is kept
            x = 3;
            //E create new variable but only for this scope
            let x = 12;
        }
        println!("x is: {}", x);
    }
}

/*
In Rust, HashMaps have no concept of default values. When you attempt to access a key that hasn't been inserted into the HashMap, you have several options:

Using get() returns an Option<&V>, which will be None if the key doesn't exist
Using get_mut() returns an Option<&mut V> for mutable access
Using entry() allows you to specify what should happen when a key is missing
 */