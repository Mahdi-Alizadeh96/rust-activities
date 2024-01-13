// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Sweet,
    Sour
}

struct Drink {
    flavor : Flavor,
    ounce : f32
}

fn print_drink (drink : Drink) {

    match drink.flavor {
        Flavor::Sour => println!("sour"),
        Flavor::Sweet => println!("sweet")
    }

    println!("{}", drink.ounce);

}

fn main() {

    let my_drink: Drink = Drink {
        flavor : Flavor::Sweet,
        ounce : 0.12
    };

    print_drink(my_drink);


}