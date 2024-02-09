// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::io;

enum Menu {
    AddBill(),
    // ViewBill,
    // RemoveBill,
    // UpdateBill,
    // BillTotal,
    // Exit
}

struct Bill {
    name : String,
    amount : i32
}

fn add_bill() {

    let mut bills: Vec<Bill> = vec![];

    let mut new_bill = Bill {
        name : "".to_owned(),
        amount : 0
    };

    let mut bill_name = String::new();

    println!("Enter bill name:");
    io::stdin().read_line(&mut bill_name).expect("Failed to read the line");
    new_bill.name = bill_name.trim().to_owned();

    let mut bill_amount = String::new();

    println!("Enter bill amount:");
    io::stdin().read_line(&mut bill_amount).expect("Failed to read the line");
    new_bill.amount = bill_amount.trim().parse().expect("Failed to read the line");

    bills.push(new_bill);

    for bill in bills {

        println!("bill name : {}, bill amount : {}", bill.name, bill.amount);
        
    }

}

fn user_input(menu_number: u32) {

    match menu_number {
        1 => add_bill(),
        _ => println!("Err")
    };

}

fn main() {

    println!("This is menu :");

    let mut selected_item = String::new();

    io::stdin().read_line(&mut selected_item).expect("Error reading line");

    // Parse the input into a u32
    let parsed_number: Result<u32, _> = selected_item.trim().parse();

    match parsed_number {
        Ok(menu_number) => user_input(menu_number),
        Err(_) => println!("Error parsing input as a number"),
    }

}