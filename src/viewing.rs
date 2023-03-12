// This module contains the functions for viewing data
use std::collections::HashMap;
use crate::common::{
    Employee,
    dept_selection
};

pub fn list_by_department(
    database: &HashMap<u32, Employee>,
    dept_list: &HashMap<u16, String>,
    dept_count: &u16
) {
    // Match the number specified by the user with the department and push the names in that department into the output vector
    println!("What department's employee list do you want to view?");
    println!("Please choose a department with the numbers below:");
    let id = dept_selection(dept_list, dept_count);
    let mut output: Vec<&String> = Vec::new();
    for (_id, entry) in database {
        if id == entry.department_id {
            output.push(&entry.name)
        }
    }

    // Sort the output vector and print the elements inside
    if output.len() == 0 {
        println!("There's current no employee in this department!");
    } else {
        output.sort();
        for name in output {
            println!("{}", name)
        }
    }
}

pub fn list_all(
    database: &HashMap<u32, Employee>, 
    dept_list: &HashMap<u16, String>, 
    employee_count: &u32
) {
    if *employee_count == 0 {
        println!("There's currently no employee in the company!");
    }

    // Create a list of departments sorted alphabetically, then convert to corresponding IDs
    let mut sorted_dept_ids: Vec<u16> = Vec::new();
    {
        let mut sorted_dept_name: Vec<&String> = Vec::new();
        for name in dept_list.values() {
            sorted_dept_name.push(name);
        }
        sorted_dept_name.sort();

        for dept_name in sorted_dept_name {
            for (id, name) in dept_list {
                if name == dept_name {
                    sorted_dept_ids.push(*id)
                }
            }
        }
    }

    for id in sorted_dept_ids {
        let mut output: Vec<&String> = Vec::new();
        for entry in database.values() {
            if id == entry.department_id {
                output.push(&entry.name);
            }
        }
        output.sort();

        let dept_name = dept_list.get(&id).unwrap();
        for name in output {
            println!("{}, {}", name, dept_name)
        }
    }
}