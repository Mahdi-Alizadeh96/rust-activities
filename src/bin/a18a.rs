// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

#[derive(Debug)]
struct Customer {
    name : String,
    age : u32
}

fn determine_purchase(customer : Customer) -> Result<Customer , String> {

    if customer.age >= 21 {

        Ok(customer)

    } else {

        Err("You can not purchase".to_owned())

    }

}

fn main() {

    let customer: Customer = Customer {
        name : "edgar".to_owned(),
        age : 25
    };

    match determine_purchase(customer) {
      Ok(customer) => println!("{} You can Buy it", customer.name),
      Err(err) => println!("{}", err)
    }

}