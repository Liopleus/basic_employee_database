// This module contains some basic functions and struct used by other modules
use std::{
    collections::HashMap,
    io::stdin,
};

pub struct Employee {
    pub department_id: u16,
    pub name: String
}

// This function takes a user input, validate if it's a number corresponding to a department in dept_list and returns that number
pub fn dept_selection(dept_list: &HashMap<u16, String>, dept_count: &u16) -> u16 {
    let mut counter = 0;
    while counter < *dept_count {
        counter += 1;
        println!("({}) {}", counter, dept_list.get(&counter).unwrap())
    }
    println!("(0) {}", dept_list.get(&0).unwrap());

    loop {
        let choice = get_number() as u16;
        match dept_list.get(&choice) {
            Some(_) => return choice,
            None => println!("Please enter a number corresponding to a department!")
        }
    }
}

// List all employees in a department, let the user choose one and return the ID of the chosen employee
pub fn employee_selection(dept_id: u16, database: &HashMap<u32, Employee>) -> u32 {
    // build a list of all employees in the selected dept, sorted by ID
    let mut employee_list: Vec<u32> = Vec::new();
    for (id, employee) in & *database {
        if employee.department_id == dept_id {
            employee_list.push(*id);
        }
    }
    employee_list.sort();

    // map numbers incrementing from 1 to IDs in the list and print the list with index, ID and name
    let mut employee_index: HashMap<u32, u32> = HashMap::new();
    let mut counter = 0;
    for id in employee_list {
        counter += 1;
        employee_index.insert(counter, id);
        println!("({}) ID:{} Name:{}", counter, id, database.get(&id).unwrap().name)
    }

    // let the user select an employee with a number
    loop {
        match employee_index.get(&get_number()) {
            Some(num) => return *num,
            None => println!("Please enter a number corresponding to an employee!")
        }
    }
}

// obtain a string with 1 or more character from the user
pub fn get_string() -> String {
    let mut input = String::new();
    loop {
        input.clear();
        stdin().read_line(&mut input).expect("Invalid Input!");
        if input.trim().len() > 0 {
            return input.trim().to_string()
        }
    }
}

// obtain a positive integer or zero from the user
pub fn get_number() -> u32 {
    let mut input = String::new();
    loop {
        input.clear();
        stdin().read_line(&mut input).expect("Please enter a valid number!");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!")
        }
    }
}
