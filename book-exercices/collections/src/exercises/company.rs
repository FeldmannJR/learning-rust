use std::{collections::HashMap, io::stdin};

/**
* Using a hash map and vectors,
* create a text interface to allow a user to add employee names to a department in a company.
* For example, “Add Sally to Engineering” or “Add Amir to Sales.”
* Then let the user retrieve a list of all people in a department or all people in the company by department,
* sorted alphabetically.
*/

type Company = HashMap<String, Vec<String>>;

pub fn read_and_process() {
    let mut company = Company::new();

    let cstdin = stdin();
    loop {
        let mut temp_string = String::new();
        println!("What you want to do?");
        println!("- Add Employee to Department");
        println!("- All employees in Department");
        println!("- All employees");
        println!("- Exit");
        println!("");
        cstdin
            .read_line(&mut temp_string)
            .expect("Cannot read terminal!");

        let temp_string = temp_string.trim();
        let words: Vec<&str> = temp_string.split(" ").collect();
        // If user send an input starting with Add and contains to
        if temp_string.starts_with("Add ") && words.get(2) == Some(&"to") {
            // Read department and person name
            let employee = words.get(1);
            let department = words.get(3);
            if let (Some(employee), Some(department)) = (employee, department) {
                add_to_department(&mut company, department, employee);
            }
        } else if temp_string == "All employees" {
            list_all_employees(&company);
        } else if temp_string.starts_with("All employees in ") {
            let department = words.get(3);
            if let Some(department) = department {
                all_employees_in_department(&company, department);
            }
        } else if temp_string.to_lowercase() == "exit" {
            break;
        }
    }
}
fn list_all_employees(company: &Company) {
    println!("Employees:");
    
    let mut all_employees: Vec<(&str, &str)> = Vec::new();
    
    //Idk how to use functional features yet, so this will be enough
    for (department, employees) in company {
        for employee in employees {
            all_employees.push((employee, department));
        }
    }
    all_employees.sort_by_cached_key(|k| k.0);
    
    for (employee, department) in all_employees {
        println!("    Name: {} Department: {}", employee, department);
    }
    print!("\n\n");
}

fn all_employees_in_department(company: &Company, department: &str) {
    println!("Employees in {}:", department);
    
    match company.get(department) {
        Some(employees) => {
            let mut sorted = employees.clone();
            sorted.sort();
            for employee in sorted {
                println!("    Name: {}", employee);
            }
        }
        None => println!("Not found any employee in the {} department!", department),
    }

    print!("\n\n");
}

fn add_to_department(company: &mut Company, department_name: &str, employee_name: &str) {
    let current_employes = company
        .entry(department_name.to_string())
        .or_insert(Vec::new());

    current_employes.push(String::from(employee_name));
    println!(
        "Inserted {} into {} department\n",
        employee_name, department_name,
    );
}
