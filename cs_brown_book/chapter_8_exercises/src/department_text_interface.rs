use std::collections::HashMap;

enum Command {
    Add { name: String, department: String },
    RetrieveFromDepartment { department: String },
    RetrieveAll,
}

fn department_interface(command: Command, database: &mut HashMap<String, Vec<String>>) {
    match command {
        Command::Add { name, department } => {
            database
                .entry(department)
                .or_insert_with(Vec::new)
                .push(name);
        }
        Command::RetrieveAll => {
            let mut departments: Vec<_> = database.iter_mut().collect();
            departments.sort_by_key(|&(department, _)| department);
            for (department, employees) in departments {
                employees.sort();
                println!("{}: {:?}", department, employees);
            }
        }
        Command::RetrieveFromDepartment { department } => {
            if let Some(employees) = database.get_mut(&department) {
                employees.sort();
                println!("Employees in {}: {:?}", department, employees);
            } else {
                println!("Department not found.");
            }
        }
    }
}

pub fn run_interface() {
    let mut input = String::new();
    let mut database: HashMap<String, Vec<String>> = HashMap::new();
    println!("Commands:");
    println!("1. Add _name_ to _department_");
    println!("2. Show people in _department_");
    println!("3. Show all people in company");
    println!("Type anything else to close.");
    loop {
        input.clear();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let words: Vec<&str> = input.trim().split_whitespace().collect();
                match words.as_slice() {
                    ["Add", name, "to", department] => {
                        // Handle add command
                        let command = Command::Add {
                            name: name.to_string(),
                            department: department.to_string(),
                        };
                        // Process command...
                        department_interface(command, &mut database);
                    }
                    ["Show", "people", "in", department] => {
                        // Handle retrieve from department command
                        let command = Command::RetrieveFromDepartment {
                            department: department.to_string(),
                        };
                        // Process command...
                        department_interface(command, &mut database);
                    }
                    ["Show", "all", "people", "in", "company"] => {
                        // Handle retrieve all command
                        let command = Command::RetrieveAll;
                        // Process command...
                        department_interface(command, &mut database);
                    }
                    _ => break,
                }
            }
            Err(error) => {
                println!("error: {error}");
                break;
            }
        }
    }
}
