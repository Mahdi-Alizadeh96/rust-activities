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
    AddBill,
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

    let new_bill = Bill {
        name : "".to_owned(),
        amount : 0
    };

    io::stdin().read_line(&mut new_bill.name);

    io::stdin().read_line(new_bill.amount);

    bills.push(new_bill);

    // bills

}

fn user_input(menu_number: u32) -> Menu {

    match menu_number {
        1 => add_bill()
    }

}

fn main() {



}