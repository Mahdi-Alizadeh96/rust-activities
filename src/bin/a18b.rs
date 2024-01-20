#![allow(dead_code)]

// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeeType {
    Maintenance,
    Marketing,
    Manager,
    Supervisor,
    KitchenStaff,
    AssemblyTechnicians
}

enum Access {
    Active,
    DeActive
}

struct Employee {
    e_type : EmployeeType,
    acc : Access
}

fn check_employee(employee : Employee) -> Result<(), String> {

    match employee.acc {
        Access::DeActive => return Err(String::from("You don't have acc")),
        _ => ()
    }

    let check = match employee.e_type {
        EmployeeType::Maintenance | EmployeeType::Marketing | EmployeeType::Manager => Ok(()),
        _ => Err("You can not enter".to_owned())
    };

    return check;

}

fn print_result(employee : Employee) -> Result<(), String> {

    check_employee(employee)?;
    println!("You have access");
    Ok(())

}

fn main() {

    let martin = Employee {
        e_type : EmployeeType::Maintenance,
        acc : Access::Active
    };

    let result = print_result(martin);
    
    match result {
        Err(e) => println!("{}", e),
        _ => ()
    }

}