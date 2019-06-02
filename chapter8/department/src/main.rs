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

enum Commands {
    Add(String, String),
    ListEmployees(String),
    ListAll,
    Quit,
    Help
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

fn read_command() -> Option<Commands> {
    use std::io;
    let mut buffer = String::new();

    println!(">");
    io::stdin().read_line(&mut buffer)
        .expect("failed to read command");

    let command = parse_command(buffer);
    command
}

fn parse_command(text: String) -> Option<Commands> {
    let words: Vec<&str> = text.trim().split_whitespace().collect();
    if words.len() == 0 {
        return None;
    }

    match words[0] {
        "quit" => Some(Commands::Quit),
        "help" => Some(Commands::Help),
        "add" => Some(Commands::Add(words[1].to_string(), words[3].to_string())),
        "list" => {
            if words.len() >= 2 {
                Some(Commands::ListEmployees(words[1].to_string()))
            } else {
                Some(Commands::ListAll)
            }
        },
        _ => {
            println!("'{}' is not a supported command. Use help to list all available commands", words[0]);
            None
        },
    }
}

fn handle_command(company: &mut Company, command: Commands) -> bool {
    match command {
        Commands::Add(name, department) => company.add_employee(&name, &department),
        Commands::ListEmployees(department) => company.list(&department),
        Commands::ListAll => company.listall(),
        Commands::Help => handle_help(),
        Commands::Quit => return false,
    }

    true
}