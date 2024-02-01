// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::prelude::*;

fn main() {

    let local: DateTime<Local> = Local::now();

    println!("{}", local);
    
    let dt = Utc.with_ymd_and_hms(2024, 1, 6, 12, 0, 9).unwrap();
    
    println!("{}", dt.format("%a %b %e %T %Y").to_string());

}