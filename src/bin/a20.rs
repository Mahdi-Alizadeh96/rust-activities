// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum PowerOptions {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate
}

fn power_message(value : PowerOptions) -> &'static str {

    let message = match value {
        PowerOptions::Hibernate => "Hibernating",
        PowerOptions::Shutdown => "Shuting down",
        PowerOptions::Reboot => "Rebooting",
        PowerOptions::Sleep => "Going to sleep",
        PowerOptions::Off => "Off"
    };

    return message;

}

fn user_input_convertor(input : &str ) -> Option<PowerOptions> {

    let result = match input.to_lowercase().as_str() {
        "off" => Some(PowerOptions::Off),
        "sleep" => Some(PowerOptions::Sleep),
        "reboot" => Some(PowerOptions::Reboot),
        "shutdown" => Some(PowerOptions::Shutdown),
        "hibernate" => Some(PowerOptions::Hibernate),
        _ => None
    };

    return result;

}

fn get_input() -> io::Result<String> {

    let mut input  = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_owned())

}

fn main() {

    match get_input() {
        Ok(input) => {
            let check_input = user_input_convertor(input.as_str()).map(|value| power_message(value));
        
            match check_input {
                Some(input) => println!("{}", input),
                _ => println!("command {} not found", input)
            };
        }
        _ => ()
    }

}
