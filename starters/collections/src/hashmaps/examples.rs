use std::{collections::HashMap, io::{self}};


pub fn median(vector: &mut Vec<i32>) -> f32 {
    if vector.len() < 1 {
        return 0.0;
    }

    vector.sort();

    let vector_length = vector.len() as f32;

    let half_index = (vector_length / 2.0).floor() as usize;
    if vector_length % 2.0 == 0.0 {
       (vector[half_index - 1] + vector[half_index]) as f32 / 2.0
    } else {
        vector[half_index] as f32
    }
}


pub fn to_pig_latin(string: &mut String) -> &mut String {
    if string.len() < 1 {
        return string;
    }

    let vowels = vec!['a', 'u', 'i', 'o', 'e'];

    let mut chars = string.chars();

    let first_char = chars.next().unwrap_or('_');

    if vowels.contains(&first_char) {
        string.push_str("-hay");
    } else {
        string.remove(0);
        string.push_str(&format!("-{first_char}ay"));
    }

    string
}

struct AvailableOptions(i32, i32);

impl AvailableOptions {
    fn between_to_str(&self) -> String {
        format!("between {} to {}", self.0, self.1)
    }
}

pub fn department_api() -> () {
    let initial_has_map = [
        ("Engineering".to_string(), Vec::new()),
        ("Sales".to_string(), vec!["Ali".to_string()]),
    ];
    let mut department_employees = HashMap::<String, Vec<String>>::from(initial_has_map);

    println!("Welcome to the Department API program!");

    println!("Please choose an operation from the menu");
    loop {
        println!("1. Add a new employee to a department");
        println!("2. Add a new department");
        println!("3. List all the employees in a department");
        println!("4. List all the departments");

        let options = AvailableOptions(1, 4);
        let operation = match  grab_number_from_stdin(&options) {
            Some(o) => o,
            None => continue,
        };

        match operation {
            1 => {
                println!("Choose the department:");
                let mut option = 1;
                let mut option_to_key_map = HashMap::new();
                for key in department_employees.keys() {
                    println!("{}. {key}", option);
                    option_to_key_map.insert(option, key);
                    option += 1;
                }
                let max: i32 = department_employees.len().try_into().unwrap_or(0); 
                let options = AvailableOptions(1, max);
                let operation = grab_number_from_stdin(&options).unwrap_or(0);
                if operation == 0 {
                    continue;
                }

                let default = &&"unknown".to_string();
                let department = option_to_key_map.get(&operation).unwrap_or(default);

                println!("department chosen: {}", department);

                println!("Type in the employee");
                let mut employee = String::new();
                io::stdin().read_line(&mut employee).expect("Please enter a valid string");

                let employee = employee.trim();

                let department = String::from((*department).clone());
                let employees = (&mut department_employees).get_mut(&department);
                if employees.is_none() {
                    continue;
                }

                employees.unwrap().push(employee.to_string());
            },
            2 => {
                println!("Please enter the name of the new department");

                let mut new_department = String::new();
                io::stdin().read_line(&mut new_department).expect("Please enter a valid string");

                let new_department = new_department.trim();

                if department_employees.contains_key(new_department) {
                    println!("This department already exists!");
                    continue;
                }

                department_employees.insert(String::from(new_department), Vec::new());
            },
            3 => {
                println!("Choose the department:");
                let mut option = 1;
                let mut option_to_key_map = HashMap::new();
                for key in department_employees.keys() {
                    println!("{}. {key}", option);
                    option_to_key_map.insert(option, key);
                    option += 1;
                }
                let max: i32 = department_employees.len().try_into().unwrap_or(0); 
                let options = AvailableOptions(1, max);
                let operation = grab_number_from_stdin(&options).unwrap_or(0);
                if operation == 0 {
                    continue;
                }

                let default = &&"unknown".to_string();
                let department = option_to_key_map.get(&operation).unwrap_or(default);

                println!("department chosen: {}", department);

                println!("Employees in {} department: {:#?}", department, department_employees[*department]);
            },
            4 => {
                let keys = department_employees.keys();
                println!("Curent departments: {:?}", keys);
            }
            _ => {
                println!("Invalid input!. Pick a number {}", &options.between_to_str());
                continue;
            }
        }

        println!("");
    }
}

fn grab_number_from_stdin(AvailableOptions(min, max): &AvailableOptions) -> Option<i32> {
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect(&format!("Please choose from the menu by entering a number from {min} to {max}."));

    let operation = operation.trim();
    let operation: i32 = match operation.parse() { 
        Ok(parsed) => {
            if parsed > *max || parsed < *min {
                println!("Your input must be between {min} and {max}");
                return None;
            } 
            parsed

        },
        Err(_) => { 
            println!("Invalid input!. Pick a number between {min} and {max}");
            return None;
        }
    };

    Some(operation)
}
