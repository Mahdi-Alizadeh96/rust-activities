// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum TicketType {
    Backstage(String),
    Vip(String),
    Standard
}

struct Ticket {
    ticket_type : TicketType,
    price : i32
}

fn main() {

    let tickets: Vec<Ticket> = vec![
        Ticket {
            ticket_type: TicketType::Backstage("David".to_owned()),
            price : 32
        },
        Ticket {
            ticket_type: TicketType::Vip("Natasha".to_owned()),
            price : 35
        },
        Ticket {
            ticket_type: TicketType::Standard,
            price : 35
        }
    ];

    for ticket in tickets {
        
        match ticket.ticket_type {
            TicketType::Backstage(name) => println!("Backstage ticket - price : {}, name : {}", ticket.price, name),
            TicketType::Vip(name) => println!("Vip ticket - price : {}, name : {}", ticket.price, name),
            _ => println!("Standard ticket - price : {}", ticket.price)
        }

    }

}