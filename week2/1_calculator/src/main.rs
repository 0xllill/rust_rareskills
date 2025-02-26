use std::fmt;

#[derive(Debug)]
struct Calculator {
    m1: i32,
    m2: i32,
}

// AdditiveOperations trait for addition and subtraction
trait AdditiveOperations {
    fn add(&self) -> i32;
    fn subtract(&self) -> i32;
}

// MultiplicativeOperations trait for multiplication and division
trait MultiplicativeOperations {
    fn multiply(&self) -> i32;
    fn divide(&self) -> Option<i32>; // Option to handle division by zero
}

// BinaryOperations trait for AND, OR, and XOR
trait BinaryOperations {
    fn bitwise_and(&self) -> i32;
    fn bitwise_or(&self) -> i32;
    fn bitwise_xor(&self) -> i32;
}

impl AdditiveOperations for Calculator {
    fn add(&self) -> i32 {
        self.m1 + self.m2
    }

    fn subtract(&self) -> i32 {
        self.m1 - self.m2
    }
    
}

impl MultiplicativeOperations for Calculator {
    fn multiply(&self) -> i32 {
        self.m1 * self.m2
    }

    fn divide(&self) -> Option<i32> {
        if self.m2 == 0 {
            None
        } else {
            Some(self.m1 / self.m2)
        }
    }
}

impl BinaryOperations for Calculator {
    fn bitwise_and(&self) -> i32 {
        self.m1 & self.m2
    }

    fn bitwise_or(&self) -> i32 {
        self.m1 | self.m2
    }

    fn bitwise_xor(&self) -> i32 {
        self.m1 ^ self.m2
    }
}



impl fmt::Display for Calculator {

    //E required method for implementing the Display trait in Rust
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let division_result = match self.divide() {
            Some(result) => result.to_string(),
            None => "Division by zero".to_string(),
        };

        write!(
            f,
            "Calculator({}, {}):\n\
             Addition: {}\n\
             Subtraction: {}\n\
             Multiplication: {}\n\
             Division: {}\n\
             AND: {}\n\
             OR: {}\n\
             XOR: {}",
            self.m1,
            self.m2,
            self.add(),
            self.subtract(),
            self.multiply(),
            division_result,
            self.bitwise_and(),
            self.bitwise_or(),
            self.bitwise_xor()
        )

    }
}


fn main() {
    // Create a new Calculator instance
    let calculator = Calculator {
        m1: 10,
        m2: 5,
    };

    // Print the calculator using the Display implementation
    println!("calculator: {}", calculator);

    // Test a division by zero scenario
    let calc_div_zero = Calculator {
        m1: 7,
        m2: 0,
    };
    
    println!("\ncalculator with division by zero: {}", calc_div_zero);

    let calc_2 = Calculator {
        m1: 7,
        m2: 3,
    };
    println!("\nUsing print_output function with division by zero:");
    print_output(&calc_2);
}


fn print_output<T>(calc: &T) where T: AdditiveOperations + MultiplicativeOperations + BinaryOperations {
    
    println!("Addition: {}", calc.add());
    println!("Subtraction: {}", calc.subtract());
    println!("Multiplication: {}", calc.multiply());

    let division_result = match calc.divide() {
        Some(result) => result.to_string(),
        None => "Division by zero".to_string(),
    };
    println!("Division: {}", division_result);

    println!("AND: {}", calc.bitwise_and());
    println!("OR: {}", calc.bitwise_or());
    println!("XOR: {}", calc.bitwise_xor());

}