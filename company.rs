// company management
// command is add or list or quit

use std::io;
use std::collections::HashMap;

fn main() {
    let mut departments = HashMap::<String, Vec<String>>::new();

    loop {
        println!("Enter a command.");
        
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line.");
        
        let mut words = command.split_whitespace();
        
        match words.next() {
            Some("help") => {
                match words.next() {
                    Some("help") => {
                        println!("help [command]: Display help for one command or all commands.");
                    },
                    Some("add") => {
                        println!("add <person> <department>: Add a person to a department, creating the department if it doesn't exist.");
                    },
                    Some("list") => {
                        println!("list [department]: List all people in a department or in the company, organized by department.");
                    },
                    Some("quit") => {
                        println!("quit: Quit the program.");
                    },
                    None => {
                        println!("There are 4 commands:");
                        println!("  help [command]: Display help for one command or all commands.");
                        println!("  add <person> <department>: Add a person to a department, creating the department if it doesn't exist.");
                        println!("  list [department]: List all people in a department or in the company, organized by department.");
                        println!("  quit: Quit the program.");
                    },
                    _ => {
                        println!("There are 4 commands:");
                        println!("  help [command]: Display help for one command or all commands.");
                        println!("  add <person> <department>: Add a person to a department, creating the department if it doesn't exist.");
                        println!("  list [department]: List all people in a department or in the company, organized by department.");
                        println!("  quit: Quit the program.");
                    }
                }
            },
            Some("add") => {
                if let Some(person) = words.next() {
                    if let Some(department) = words.next() {
                        let people = &mut departments.entry(department.to_string()).or_insert(Vec::new());
                        people.push(person.to_string());
                        println!("{person} added to {department}.");
                    } else {
                        println!("Missing a department to add the person to.");
                    }
                } else {
                    println!("Missing a person to add.");
                }
            },
            Some("list") => {
                if let Some(department) = words.next() {
                    if let Some(people) = departments.get(department) {
                        println!("People in {department}:");
                        for person in people {
                            println!("  {person}");
                        }
                    } else {
                        println!("No department found: {department}");
                    }
                } else {
                    for (department, people) in &departments {
                        println!("People in {department}:");
                        for person in people {
                            println!("  {person}");
                        }
                    }
                }
            },
            Some("quit") => break,
            _ => {
                println!("Invalid command. Try `help`");
            }
        }
    }
}