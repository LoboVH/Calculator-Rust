use std::io;

// Define the Operation enum with appropriate variants and values
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Write the calculate function signature
fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => x / y,
    }
}

fn main() {
    loop {
         // Prompt the user to input the first number, operation, and second number
        println!("Enter the first number:");
        let mut first_number = String::new();
        io::stdin().read_line(&mut first_number).expect("Failed to read line");
        let first_number: f64 = first_number.trim().parse().expect("Please enter a valid number");

        println!("Enter the operation [    + (add), - (subtract), * (multiply), / (divide),  STOP    ]:");
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Failed to read line");
        let operation: Operation = match operation.trim() {
            "+" => Operation::Add(first_number, 0.0),
            "-" => Operation::Subtract(first_number, 0.0),
            "*" => Operation::Multiply(first_number, 0.0),
            "/" => Operation::Divide(first_number, 0.0),
            _ => panic!("Invalid operation. Please enter [    + (add), - (subtract), * (multiply), / (divide)    ]."),
        };

        println!("Enter the second number:");
        let mut second_number = String::new();
        io::stdin().read_line(&mut second_number).expect("Failed to read line");
        let second_number: f64 = second_number.trim().parse().expect("Please enter a valid number");

        // Create an Operation enum instance with the parsed input values
        let operation = match operation {
            Operation::Add(x, _) => Operation::Add(x, second_number),
            Operation::Subtract(x, _) => Operation::Subtract(x, second_number),
            Operation::Multiply(x, _) => Operation::Multiply(x, second_number),
            Operation::Divide(x, _) => Operation::Divide(x, second_number),
        };

        // Call the calculate function with the created Operation enum instance
        let result = calculate(operation);

        // Print the result to the console
        println!("Result: {} \n\n\n", result);
        println!("Proceed to perform another calculation");
        println!("Would you like to continue? (y/n) \n\n");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_lowercase();

        if input == "n" {
            println!("Thank you for using the calculator, Goodbye!");
            std::process::exit(0);
        }
    }
    
}