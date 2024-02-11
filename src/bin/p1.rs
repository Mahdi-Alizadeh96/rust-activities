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

struct Bill {
    name : String,
    amount : f64
}

struct Bills {
    bills : Vec<Bill>
}

impl Bills {

    fn new() -> Self {
        Bills { bills : Vec::new()}
    }
    
    fn add_bill(&mut self, name : String, amount : f64) {
        
        let new_bill = Bill {
            name,
            amount
        };

        self.bills.push(new_bill);

    }

    fn view_bills (&self) {

        println!("--------------");

        for bill in &self.bills {
    
            println!("bill name : {}bill amount : {}", bill.name, bill.amount);
            println!("--------------")
            
        }

    }

    fn remove_bills(&mut self, name : String) {

        let check_exist = self.bills.iter().position(|item| item.name == name);

        match check_exist {
            Some(index) => {
                self.bills.remove(index);
            },
            None => println!(">> Bill name dose not exist !")
        }

    }

    fn update_bills(&mut self, bill_name : String, updated_amount : f64 ) {

        let check_exist = self.bills.iter().position(|item| item.name == bill_name);

        match check_exist {
            Some(_index) => {

                for bill in &mut self.bills {

                    if bill.name == bill_name {

                        bill.amount = updated_amount;

                    }

                }

            },
            None => println!(">> Bill name dose not exist !")
        }

    }

    fn bills_total(&self) {

        let mut total: f64 = 0.0;

        for bill in &self.bills  {
            
            total = total + bill.amount;

        };

        println!("total amount of bills is : {}", total);

    }
    
}

fn user_input() -> u32 {

    let mut selected_item = String::new();

    io::stdin().read_line(&mut selected_item).expect(">> Error reading line");

    // Parse the input into a u32
    let parsed_number: Result<u32, _> = selected_item.trim().parse();
    
    match parsed_number {
        Ok(menu_number) => {

            return menu_number;

        },
        Err(_) => {
            println!(">> Menu number is wrong");
            user_input()
        },
    }

}

fn show_menu() {

    println!(r#"
please select menu number

== Manage Bills ==
1.Add bill
2.View bills
3.Remove bill
4.Update bill
5.Bill total

6.Show Menu
7.exit
    "#);

}

fn main() {

    let mut bills = Bills::new();

    show_menu();

    loop {

        match user_input() {
            1 => {

                let mut bill_name = String::new();

                println!("Enter bill name:");
                io::stdin().read_line(&mut bill_name).expect("Failed to read the line");
            
                let mut bill_amount = String::new();
            
                println!("Enter bill amount:");
                io::stdin().read_line(&mut bill_amount).expect("Failed to read the line");
                let amount = bill_amount.trim().parse::<f64>().unwrap_or(0.0);

                bills.add_bill(bill_name, amount);

                bills.view_bills();

                println!("Press 6 for show menu");

            },
            2 => {

                bills.view_bills();

                println!("Press 6 for show menu");

            },
            3 => {

                println!("Print bill name :");

                let mut bill_name = String::new();

                io::stdin().read_line(&mut bill_name).expect("Failed to read the line");

                bills.remove_bills(bill_name);

                bills.view_bills();

                println!("Press 6 for show menu");

            },
            4 => {

                println!("Print bill name :");

                let mut bill_name = String::new();

                io::stdin().read_line(&mut bill_name).expect("Failed to read the line");

                let mut bill_amount = String::new();

                println!("Enter new bill amount:");
                io::stdin().read_line(&mut bill_amount).expect("Failed to read the line");
                let amount = bill_amount.trim().parse::<f64>().unwrap_or(0.0);

                bills.update_bills(bill_name, amount);

                bills.view_bills();

                println!("Press 6 for show menu");

            },
            5 => {

                bills.bills_total();

                println!("Press 6 for show menu");

            }
            6 => {

                show_menu();

            },
            7 => {

                println!("Good bay");
                break;

            },
            _ => {

                println!("Menu number is invalid");

            }
        }
        
    }



}