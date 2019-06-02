// Using a hash map and vectors, create a text interface to allow a user to add
// employee names to a department in a company.

// For example, “Add Sally to Engineering” or “Add Amir to Sales.”

// Then let the user retrieve a list of all people in a department or all
// people in the company by department, sorted alphabetically.

use std::collections::HashMap;

struct Company(HashMap<String, Vec<String>>);

impl Company {
    fn new() -> Company {
        let map = HashMap::new();
        Company(map)
    }

    fn add_employee(&mut self, name: &str, department: &str) {
        let map = &mut self.0;
        
        let name = String::from(name);
        let department = String::from(department);

        let vector = map.entry(department)
            .or_insert(Vec::new());

        vector.push(name);
        vector.sort_unstable();
    }
    
    fn employees_from(&self, department: &str) -> Option<&Vec<String>> {
        let department = self.0.get(department);
        department
    }

    fn list(&self, department: &str) {
        let employees = self.employees_from(department);
        println!("Employees of {}", department);
        for employee in employees.unwrap() {
            println!("- {}", employee);
        }
    }


    fn listall(&self) {
        let mut result: Vec<&str> = Vec::new();

        for (_, list) in &self.0 {
            for employee in list.iter() {
                result.push(employee);
            }
        }

        result.sort_unstable();

        println!("Employees");
        for employee in result {
            println!("- {}", employee);
        }
    }
}

enum Command {
    Add(String, String),
    ListEmployees(String),
    ListAll,
    Quit,
    Help
}

impl Command {
    fn parse(text: String) -> Option<Command> {
        let words: Vec<&str> = text.trim().split_whitespace().collect();
        if words.len() == 0 {
            return None;
        }

        match words[0] {
            "quit" => Some(Command::Quit),
            "help" => Some(Command::Help),
            "add" => Some(Command::Add(words[1].to_string(), words[3].to_string())),
            "list" => {
                if words.len() >= 2 {
                    Some(Command::ListEmployees(words[1].to_string()))
                } else {
                    Some(Command::ListAll)
                }
            },
            _ => {
                println!("'{}' is not a supported command. Use help to list all available commands", words[0]);
                None
            },
        }
    }
}

fn handle_help() {
    println!("Valid commands are:");
    println!("add <name> to <department>");
    println!("list <department>");
    println!("list");
    println!("help");
    println!("quit");
}

fn main() {
    let mut company = Company::new();
    let mut is_running = true;

    while is_running {
        let command = read_command();
        if let Some(cmd) = command {
            is_running = handle_command(&mut company, cmd);
        }
    }
}

fn read_command() -> Option<Command> {
    use std::io;
    let mut buffer = String::new();

    println!(">");
    io::stdin().read_line(&mut buffer)
        .expect("failed to read command");

    Command::parse(buffer)
}

fn handle_command(company: &mut Company, command: Command) -> bool {
    match command {
        Command::Add(name, department) => company.add_employee(&name, &department),
        Command::ListEmployees(department) => company.list(&department),
        Command::ListAll => company.listall(),
        Command::Help => handle_help(),
        Command::Quit => return false,
    }

    true
}