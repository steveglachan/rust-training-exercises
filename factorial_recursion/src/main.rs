use std::io::Write;

// Used a u32 instead of i32 since we don't support negatives
fn factorial(x: u32) -> u32 {
    if x == 0 {
        1
    } else {
        x * factorial(x - 1)
    }
}

fn get_user_input() -> String {
    // Request user input:
    print!("\nStart number input: ");
    let mut user_input: String = String::new();
    std::io::stdout().flush().map_err(|err| println!("Error flushing output. {:?}", err)).ok(); // Ignore Error result
    std::io::stdin().read_line(&mut user_input).map_err(|err| println!("Error reading input. {:?}", err)).ok(); // Ignore Error result
    return user_input;
}

fn user_input_is_valid(user_input: String) -> bool {
    return user_input.trim().parse::<u32>().is_ok();
}

fn parse_user_input(user_input: String) -> u32 {
    return user_input.trim().parse::<u32>().unwrap();
}

fn main() {

    // Request user input:
    let user_input: String = get_user_input();
    if user_input_is_valid(user_input.clone()) {
        let number: u32 = parse_user_input(user_input);
        let factorial_answer: u32 = factorial(number);
        println!(
"----------------------------------------------
Factorial of {}: {}
----------------------------------------------", 
            number, 
            factorial_answer
        );
    } else {
        println!(
"----------------------------------------------
ERROR:
Input is not a positive number.
Please run the app again using `cargo run`.
----------------------------------------------"
        );
    }
}