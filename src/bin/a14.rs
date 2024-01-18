// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name : String,
    color : String,
    age : u32
}

fn print_person_details(person : Person) {

    println!("name : {} ,color : {} , age : {}", person.name, person.color, person.age);

}

fn main() {

    let people: Vec<Person> = vec![
        Person {
            name : "David".to_owned(),
            color : "red".to_owned(),
            age : 8
        },
        Person {
            name : "Samuel".to_owned(),
            color : "black".to_owned(),
            age : 17
        },
        Person {
            name : "Sofia".to_owned(),
            color : "pink".to_owned(),
            age : 3
        }
    ];

    for person in people {
        
        if person.age <= 10 {

            print_person_details(person);

        }

    }

}