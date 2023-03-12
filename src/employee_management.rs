// This module contains the functions for adding, renaming, reassigning and removing employees
use std::collections::HashMap;
use crate::common::{
    Employee,
    employee_selection,
    dept_selection,
    get_string
};

pub fn add_employee(
    database: &mut HashMap<u32, Employee>,
    dept_list: &HashMap<u16, String>,
    dept_count: &u16,
    employee_count: &mut u32
) {
    // ask user for the name of the employee 
    println!("What's the name of this employee?");
    let name = get_string();

    // find an unused employee ID
    let mut used_id: Vec<u32> = Vec::new();
    for id in &mut database.keys() {
        used_id.push(*id)
    }
    used_id.sort();
    let mut id_to_be_assigned: u32 = used_id[0];
    for id in used_id {
        if id_to_be_assigned == id {
            id_to_be_assigned += 1
        }
    }

    // add the employee to the database
    println!("What department does this employee belong to?");
    let id = dept_selection(dept_list, dept_count);
    let new_employee = Employee{
        department_id: id,
        name: name
    };
    println!(
        "An employee named {} has been added to the {} department.",
        new_employee.name,
        dept_list.get(&id).unwrap()
    );
    database.insert(id_to_be_assigned, new_employee);
    *employee_count += 1;
}

pub fn rename_employee(
    database: &mut HashMap<u32, Employee>,
    dept_list: &HashMap<u16, String>,
    dept_count: &u16,
    employee_count: &u32
) {
    if *employee_count == 0 {
        println!("There's no employee to rename!");
        return
    }

    println!("What department does the employee you want to rename belong to?");
    let dept_id = dept_selection (dept_list, dept_count);
    let mut counter = 0;

    // return if there's no employee in the selected department
    for (_id, entry) in &*database {
        if entry.department_id == dept_id {
            counter += 1
        }
    }
    match counter {
        0 => {
            println!("There's currently no employee in this department!");
            return
        }
        _ => ()
    }

    let choice = employee_selection(dept_id, database);
    println!("Please enter the new name of the employee:");
    let new_name = get_string();
    println!("The employee named {} in the {} department has been renamed to {}.", 
    database.get(&choice).unwrap().name,
    dept_list.get(&dept_id).unwrap(),
    new_name
    );
    database.insert(choice, Employee { 
        department_id: dept_id, 
        name: new_name 
    }
    );
}

pub fn reassign_employee(
    database: &mut HashMap<u32, Employee>,
    dept_list: &HashMap<u16, String>,
    dept_count: &u16
) {
    if *dept_count == 0 {
        println!("There's currently no department in the database!");
    }
    
    // ask for the employee to ressign to another department
    println!("What department does the employee you want to reassign belong to?");
    let original_dept = dept_selection(dept_list, dept_count);
    println!("Which employee do you want to reassign to another department?");
    let employee_id = employee_selection(original_dept, database);
    println!("Which department do you want to reassign the employee to?");
    let target_dept = dept_selection(dept_list, dept_count);

    let updated_employee = Employee {
        name: database.get(&employee_id).unwrap().name.clone(),
        department_id: target_dept
    };

    database.insert(employee_id, updated_employee);
    println!("The employee named {} with ID of {} is moved from {} department to {} department.",
    database.get(&employee_id).unwrap().name,
    employee_id,
    dept_list.get(&original_dept).unwrap(),
    dept_list.get(&target_dept).unwrap()
    )
}

pub fn remove_employee(
    database: &mut HashMap<u32, Employee>,
    dept_list: &HashMap<u16, String>,
    dept_count: &u16,
    employee_count: &mut u32
) {
    println!("What department does the employee you want to remove belong to?");
    let dept_id = dept_selection(dept_list, dept_count);

    println!("Which employee do you want to remove?");
    let employee_id = employee_selection(dept_id, database);

    let removed_employee = database.remove(&employee_id).unwrap();
    println!("The employee named {} from the {} department has been removed.",
    removed_employee.name,
    dept_list.get(&removed_employee.department_id).unwrap()
    );
    *employee_count -= 1;
    println!("{}",employee_count)
}