use std::io::Write;

fn tell_temperature(temp: f32) {
    let temp_desc: &str = if temp <=0.0 {
        "freezing"
    } else if temp <= 10.0 {
        "cold"
    } else if temp >= 11.0 && temp <= 25.0 {
        "nice"
    } else if temp >= 26.0 && temp <= 30.0 {
        "warm"
    } else {
        "hot"
    };
    println!(
"------------------------------------------
It's {}, at {} degrees celcius.
------------------------------------------", 
        temp_desc, 
        temp
    );
}

fn get_user_input() -> String {
    // Request user input:
    print!("\nTemperature input: ");
    let mut user_input: String = String::new();
    std::io::stdout().flush().map_err(|err| println!("Error flushing output. {:?}", err)).ok(); // Ignore Error result
    std::io::stdin().read_line(&mut user_input).map_err(|err| println!("Error reading input. {:?}", err)).ok(); // Ignore Error result
    return user_input;
}

fn user_input_is_valid(user_input: String) -> bool {
    return user_input.trim().parse::<f32>().is_ok();
}

fn parse_user_input(user_input: String) -> f32 {
    return user_input.trim().parse::<f32>().unwrap();
}

fn main() {
    
    // Request user input:
    let user_input: String = get_user_input();
    if user_input_is_valid(user_input.clone()) {
        let temperature = parse_user_input(user_input.clone());
        tell_temperature(temperature);
    } else {
        println!(
"----------------------------------------------
ERROR:
Input is not a positive or negative number.
Please run the app again using `cargo run`.
----------------------------------------------"
        );
    }
}
