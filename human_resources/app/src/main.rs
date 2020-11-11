use std::collections::HashMap;

fn main() {
    let mut company = HashMap::new();

    let employee = "Sally";
    let department = "Engineering";
    company
        .entry(&department)
        .or_insert_with(Vec::new)
        .push(employee);

    let employee2 = "Amir";
    let department2 = "Sales";
    company
        .entry(&department2)
        .or_insert_with(Vec::new)
        .push(employee2);

    let employee2 = "John";
    let department2 = "Sales";
    company
        .entry(&department2)
        .or_insert_with(Vec::new)
        .push(employee2);

    println!("{:#?}", company);

    let department_lookup = "Sales";
    match company.get(&department_lookup) {
        Some(employees) => println!(
            "Employees of Department '{}': {:?}",
            department_lookup, employees,
        ),
        None => println!("Department '{}' does not exist.", department_lookup),
    }
}
