// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn sum(a : i32, b: i32) -> i32 {
    a + b
}

fn display_result(a : i32, b: i32, result : i32) {

    println!("The result of sum {:?} and {:?} is {:?}", a, b, result);

}

fn main() {

    let number1 : i32 = 3;
    let number2 : i32 = 5;

    let my_number: i32 = sum(number1, number2);

    display_result(number1, number2, my_number)

}
