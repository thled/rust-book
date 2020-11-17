use std::{collections::HashMap, io};

fn main() {
    let mut company = HashMap::new();

    let employee = "John";
    let department = "Engineering";
    company
        .entry(department)
        .or_insert_with(Vec::new)
        .push(employee);

    loop {
        println!("To add an employee: Add EMPLOYEE to DEPARTMENT");
        println!("To list employees of department: List DEPARTMENT");
        println!("To list all employees: All");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        let mut words = input.split_whitespace();
        match words.next() {
            Some(word) => match word {
                "All" => {
                    for (_department, employees) in &mut company {
                        employees.sort_unstable();
                    }
                    println!("{:#?}", company);
                }
                "List" => {
                    let department_lookup = match words.next() {
                        Some(department) => department,
                        None => "Engineering",
                    };
                    match company.get(&department_lookup) {
                        Some(employees) => println!(
                            "Employees of Department '{}': {:?}",
                            department_lookup, employees,
                        ),
                        None => println!("Department '{}' does not exist.", department_lookup),
                    }
                }
                "Add" => {
                    let employee = words.next().unwrap();
                    words.next();
                    let department = words.next().unwrap();

                    // todo: not working
                    // company
                    //     .entry(department)
                    //     .or_insert_with(Vec::new)
                    //     .push(employee);

                    println!("{} added to {}", employee, department);
                }
                _ => println!("Please use a pattern from above."),
            },
            None => println!("No words detected!"),
        }

        println!("");
    }
}
