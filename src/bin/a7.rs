// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Blue,
    Green
}

fn print_color (color : Color) {

    match color {
        Color::Blue => println!("blue"),
        Color::Red => println!("red"),
        Color::Green => println!("green"),
    }

}

fn main() {

    let color: Color = Color::Blue;

    print_color(color);

}