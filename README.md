# Rust Activities (zero to mastery)

## [Rust Programming: The Complete Developer's Guide](https:-rotomastery.io/courses/learn-rust/)
---
### Activity 1 - Functions

- Displays your first and last name

### Activity 2 - Basic arithmetic

- Displays the result of the sum of two numbers

### Activity 3 - Flow control using if..else

- a. Display a message based on the value of a boolean variable
- b. Display ">5", "<5", or "=5" based on the value of a variable

### Activity 4 - Decision making with match

- a. Display "it's true" or "it's false" based on the value of a variable
- b. Display "one", "two", "three", or "other" based on whether the value of a variable is 1, 2, 3, or some other number, respectively

### Activity 5 - Basic arithmetic

- Display "1" through "4" in the terminal

### Activity 6 - Looping using the while statement

- Counts down from 5 to 1, displays the countdown in the terminal, then prints "done!" when complete.

### Activity 7 - Working with an enum

- Prints the name of a color to the terminal

### Activity 8 - Organizing similar data using structs

- Print the flavor of a drink and it's fluid ounces

### Activity 9 - Data management using tuples

- Print whether the y-value of a cartesian coordinate is greater than 5, less than 5, or equal to 5

### Activity 10 - Working with expressions

- Print "its big" if a variable is > 100
- Print "its small" if a variable is <= 100

### Activity 11 - Ownership

- Print out the quantity and id number of a grocery item

### Activity 12 - Implementing functionality with the impl keyword

- Print the characteristics of a shipping box Must include dimensions, weight, and color

### Activity 13 - Vectors

- Print 10, 20, "thirty", and 40 in a loop Print the total number of elements in a vector

### Activity 14 - Strings

- Print out the name and favorite colors of people aged 10 and under

### Activity 15 - Advanced match

- Print out a list of tickets and their information for an event
- Tickets can be Backstage, Vip, and Standard
- Backstage and Vip tickets include the ticket holder's name
- All tickets include the price

### Activity 16 - Option

- Print out the details of a student's locker assignment
- Lockers use numbers and are optional for students

### Activity 17 - Browsing standard library documentation

- Print a string in lowercase and uppercase

### Activity 18 - Result & the question mark operator

- a.
    - Determine if a customer is able to make a restricted purchase
    - Restricted purchases require that the age of the customer is at least 21
- b.
    - Determine if an employee can access a building using a digital keycard
    - Employees that can access the building are:
        - Maintenance crews
        - Marketing department employees
        - Managers
    - Other employees that work at the company are:
        - Line supervisors
        - Kitchen staff
        - Assembly technicians
    - Ensure that terminated employees cannot access the building regardless of their position

### Activity 19 - HashMap

- Print the name and number of items in stock for a furniture store
- If the number of items is 0, print "out of stock" instead of 0
- The store has:
    - 5 Chairs
    - 3 Beds
    - 2 Tables
    - 0 Couches
- Print the total number of items in stock

### Activity 20 - User input

- Verify user input against pre-defined keywords
- The keywords represent possible power options for a computer:
    - Off
    - Sleep
    - Reboot
    - Shutdown
    - Hibernate
- If the user enters one of the keywords, a message should be printed to the console indicating which action will be taken
    - Example: If the user types in "shutdown" a message should display such as "shutting down"
- If the keyword entered does not exist, an appropriate error message should be displayed

### Activity 21 - Map combinator

- Given a user name, create and print out a User struct if the user exists

### Activity 22 - Testing

- Write tests for the existing program to ensure proper functionality

### Activity 23 - Option combinators

- Use combinators as described in the functions: part_1, part_2, and part_3

### Activity 24 - Iterator

- Triple the value of each item in a vector.
- Filter the data to only include values > 10.
- Print out each element using a for loop.

### Activity 25 - Traits

- Calculate the perimeter of a square and triangle
    - The perimeter of a square is the length of any side*4.
    - The perimeter of a triangle is a+b+c where each variable represents the length of a side.
- Print out the perimeter of the shapes

### Activity 26 - Modules

- a. External crates
    - Display the current date and time
- b. Inline Modules
    -  The existing program is complete, but all the code exists in a single module. This code can benefit from being organized into multiple modules.
- c. External Modules
    - The existing program is complete, but all the code exists in a single module. This code can benefit from being organized into multiple external modules.

### Activity 29 - Generics & Functions

- Create a function that accepts the Priority trait as a generic parameter
    - The function should print out the guest and their priority
- Use the function with at least two different guests

# Project 1 - Interactive bill manager

- Create a command line bills/expenses manager that runs interactively. This mini project brings together many of the concepts learn thus far into a single application.
- The user stories/requirements are split into stages. Fully implement each stage as a complete working program before making changes for the next stage. Leverage the compiler by using `cargo check --bin p1` when changing between stages to help identify adjustments that need to be made.