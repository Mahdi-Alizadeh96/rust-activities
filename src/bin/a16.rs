// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name : String,
    locker_assignment : Option<i32>
}

fn main() {

    let student: Student = Student {
        name : "Samuel".to_owned(),
        locker_assignment : None
    };

    match student.locker_assignment {
        Some(code) => println!("Code for : {} is : {}", student.name, code),
        None => println!("No Code for : {}", student.name)
    }

}