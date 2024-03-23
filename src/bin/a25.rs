// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn calculate_perimeter(&self) -> f64;
}

struct Square {
    length : f64
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> f64 {
        self.length * 4.0       
    }
}

struct Triangle {
    a : f64,
    b : f64,
    c : f64
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

fn main() {

    let square = Square  {
        length : 87.0
    };

    println!("perimeter of square is {}", square.calculate_perimeter());   

    let triangle = Triangle  {
        a : 6.0,
        b : 4.2,
        c : 5.9
    };

    println!("perimeter of triangle is {}", triangle.calculate_perimeter()); 

}