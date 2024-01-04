// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn display_name(name : &str) -> String {

    format!("My name is {}", name)

}

fn display_last_name(name : &str) -> String {

    format!("my last name is {}", name)

}

fn main() {

    let name: String = display_name("Mahdi");
    let last_name: String = display_last_name("Alizadeh");

    println!("{} and {}", name, last_name);

}