# Learning Rust Exercise: Processing CLI user input
---
**User Interface:**
- Console / CLI

**Program purpose:**
- Prompt user (at console) to input a numeric temperature value (celcius).
- Input value can be a whole number or decimal, positive or negative.
- Parse and validate input value (f32).
- Output statement about the input temperature in the console.

_Note:_ 
- Alternative approaches initially taken have been included below for reference.
- Earlier iterations of the solution used i32 type values (non decimal).

**To run:**
- `> cargo run`

---

# Evolution of solutions
The following blocks of code were initially used while experimenting with solutions.
They have since been abandoned but reserved as developer notes.

## v4.0: Current
See `src/main.rs` for current solution.
* User input prompted (positive and negative floats accepted).
* User input read.
* User input validated.
* Output generated.

## v3.0: Outdated.
* User input prompted.
* User input read.
* No numeric validation on user input.
```
// Executed within main()
print!("Enter Number: ");
let mut number = String::new();
std::io::stdout().flush();
std::io::stdin().read_line(&mut number);
let temperature: i32 = number.trim().parse().unwrap(); // i32 allows negative numbers.
tell_temperature(temperature);
```
## v2.0: Outdated.
* CLI input accepted in initial run command (`cargo run [temperature arg]`).
* No trim or negative numeric validation on user input.
```
// Executed within main()
// Requires: `use std::env;`
let mut temperature: i32 = num; // mutable, default
// Handle CLI param for temperature:
let args: Vec<String> = env::args().collect();
temperature = if ( args.len() > 1 ) && ( args[1].chars().all(char::is_numeric) ) {
    args[1].parse().unwrap()
} else {
    temperature
};
tell_temperature(temperature);
```
## v1.0: Outdated.
* CLI input accepted in initial run command (`cargo run [temperature arg]`).
* No trim or numeric validation on user input.
```
// Executed within main()
// Requires: `use std::env;`
// Handle CLI param for temperature:
let args: Vec<String> = env::args().collect();
if args.len() > 1 {
    let temperature: i32 = args[1].parse().unwrap();
    tell_temperature(temperature);
} else {
    tell_temperature(default_temperature);
}
```