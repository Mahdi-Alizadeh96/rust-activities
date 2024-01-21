// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {

    let store = HashMap::from([
        ("chairs", 5),
        ("beds", 3),
        ("tables", 2),
        ("couches", 0)
    ]);

    let mut total  = 0;

    for (key, &value) in store.iter() {

        if value == 0 {

            println!("{} : out of stock", key);

        } else {

            println!("{} : {}", key, value);

            total += value;

        }

    }

    println!("total amount of items is : {}", total);

}