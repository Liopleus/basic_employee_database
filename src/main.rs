mod common;
mod dept_management;
mod employee_management;
mod viewing;
use std::collections::HashMap;

fn main() {
    println!("This program allows you to build, manage and view an employee database containing the name and the department.");
    let mut dept_count: u16 = 0;
    let mut employee_count: u32 = 0;
    let mut dept_list: HashMap<u16, String> = HashMap::new();
    let mut employee_data: HashMap<u32, common::Employee> = HashMap::new();
    dept_list.insert(0, "No Department".to_string());

    loop {
        let option_add_employee = 1;
        let option_create_dept = 2;
        let option_list_by_dept = 3;
        let option_list_all = 4;
        let option_rename_employee = 5;
        let option_reassign_employee = 6;
        let option_remove_employee = 7;
        let option_rename_department = 8; 
        let option_remove_department = 9; 
        let option_exit = 0;

        println!("Choose what you want to do:");
        println!("({}) Add an employee to the database", option_add_employee);
        println!("({}) Add a new department to the database",option_create_dept);
        println!("({}) List the employees in a department",option_list_by_dept);
        println!("({}) List all employees in the company", option_list_all);
        println!("({}) Rename an employee in a selected department", option_rename_employee);
        println!("({}) Reassign an employee to another department", option_reassign_employee);
        println!("({}) Remove an employee from a selected department", option_remove_employee);
        println!("({}) Rename a department", option_rename_department);
        println!("({}) Remove a department", option_remove_department);
        println!("({}) Exit the program", option_exit);

        let mode = common::get_number();
        if mode == option_add_employee {
            employee_management::add_employee(&mut employee_data, &dept_list, &dept_count, &mut employee_count)
        } else if mode == option_create_dept {
            dept_management::create_dept(&mut dept_count, &mut dept_list)    
        } else if mode == option_list_by_dept {
            viewing::list_by_department(&employee_data, &dept_list, &dept_count)
        } else if mode == option_list_all {
            viewing::list_all(&employee_data, &dept_list, &employee_count)
        } else if mode == option_rename_employee {
            employee_management::rename_employee(&mut employee_data, &dept_list, &dept_count, &employee_count)    
        } else if mode == option_reassign_employee {
            employee_management::reassign_employee(&mut employee_data, &dept_list, &dept_count)
        } else if mode == option_remove_employee {
            employee_management::remove_employee(&mut employee_data, &dept_list, &dept_count, &mut employee_count)
        } else if mode == option_rename_department {
            dept_management::rename_department(&mut dept_list, &dept_count)
        } else if mode == option_remove_department {
            dept_management::remove_department(&mut employee_data, &mut dept_list, &mut dept_count)
        } else if mode == option_exit {
            std::process::exit(0);
        } else {
            println!("Please enter a number corresponding to one of the options!");
            continue;
        }
    }
}