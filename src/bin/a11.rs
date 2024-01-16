// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    quantity : i32,
    id_number : i32
}

fn display_quantity (grocery : &Grocery) {
    
    println!("{}", grocery.quantity);

}

fn display_id_number (grocery : &Grocery) {
    
    println!("{}", grocery.id_number);

}

fn main() {

    let milk = Grocery {
        quantity : 3,
        id_number : 10003
    }; 

    display_quantity(&milk);

    display_id_number(&milk);

}