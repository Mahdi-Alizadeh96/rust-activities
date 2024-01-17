// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Blue
}

impl Color {
    
    fn print_color(&self) {
        match self {
            Color::Blue => println!("blue"),
            Color::Red => println!("red")
        }
    }

}

struct Dimensions {
    x : f64,
    y : f64,
    z : f64
}

impl Dimensions {
    
    fn print_dimensions (&self) {

        println!("x : {}, y : {}, z : {}", self.x, self.y, self.z);

    }

}

struct ShippingBox {
    color : Color,
    dimensions : Dimensions,
    weight : f64
}

impl ShippingBox {
    
    fn new_box (color : Color, dimensions : Dimensions, weight : f64) -> Self {
        Self {
            weight,
            color,
            dimensions
        }
    }

    fn print(&self) {

        self.color.print_color();

        self.dimensions.print_dimensions();

        println!("wight : {}", self.weight);

    }

}

fn main() {

    let my_box_dimensions = Dimensions {
        x : 54.3,
        y : 65.3,
        z : 7.2
    };

    let my_box = ShippingBox::new_box(Color::Blue, my_box_dimensions, 62.2);
    

    ShippingBox::print(&my_box);

}