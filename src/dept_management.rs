// This module contains the function used for creating, renaming and removing departments
use crate::common::{
    Employee,
    dept_selection,
    get_string
};
use std::collections::HashMap;

pub fn create_dept(
    count: &mut u16, 
    list: &mut HashMap<u16, String>
) {
    println!("Please enter the name of the department:");
    let name = get_string();

    // find an unused dept id
    let mut sorted_dept_ids = Vec::new();
    for id in list.keys() {
        sorted_dept_ids.push(*id)
    }
    let mut id_to_assign = sorted_dept_ids[0];
    for id in sorted_dept_ids {
        if id == id_to_assign {
            id_to_assign += 1
        }
    }

    println!(
        "The {} department has been added to the database.",
        name
    );
    list.insert(id_to_assign, name);
    *count += 1;

}

pub fn rename_department(
    dept_list: &mut HashMap<u16, String>,
    dept_count: &u16
) {
    println!("Which department do you want to rename?");
    let selected_dept = dept_selection(dept_list, dept_count);
    if selected_dept == 0 {
        println!("This department can't be renamed!");
        return
    }

    println!("Please enter the new name of the department:");
    let new_name = get_string();
    println!("The {} department has been renamed to {}.",dept_list.get(&selected_dept).unwrap(), new_name);
    dept_list.insert(selected_dept, new_name);
}

pub fn remove_department(
    database: &mut HashMap<u32, Employee>,
    dept_list: &mut HashMap<u16, String>,
    dept_count: &mut u16
) {
    println!("Which department do you want to remove?");
    let dept_id = dept_selection(dept_list, dept_count);
    if dept_id == 0 {
        println!("This department can't be removed!");
        return
    }

    let removed_department = dept_list.remove(&dept_id).unwrap();
    *dept_count -= 1;
    println!("The department named {} has been removed from the company.", removed_department);
    // reassign the employee in the deleted department to have no department
    for entry in database.values_mut() {
        if entry.department_id == dept_id {
            entry.department_id = 0
        }
    }
    println!("All employee previously in this department has been reassigned to have no department.")
}