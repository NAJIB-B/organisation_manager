pub mod user_input_sorter;

use std::{collections::HashMap, io};

fn main() {
    //instructions
    println!("welcome to oraganization manager");
    println!("To add a staff to a department ");
    println!("Type: Add <staff_name> to <department_name>");
    println!("example: Add najib to engineering");
    println!("To remove a staff from a department ");
    println!("Type: Remove <staff_name> from <department_name>");
    println!("To get list of staffs in a department");
    println!("Type: List of <department_name>");
    println!("To get List of all staffs");
    println!("Type: List of staffs");
    println!("----------- Starting -------------");

    let mut organisation_hashmap: HashMap<String, String> = HashMap::new();
    let mut list_of_staff: Vec<String> = Vec::new();
    let mut list_of_department: Vec<String> = Vec::new();
    loop {
        let mut user_input = String::new();
        println!("Please input your command");
        io::stdin()
            .read_line(&mut user_input)
            .expect("could not process your input");

        let borrowed_user_input = &user_input;

        let split = borrowed_user_input.split_whitespace();
        let user_input_vector: Vec<String> = split.map(|s| s.to_string()).collect();

        let user_input_length = user_input_sorter::number_of_word_in_string(&user_input);

        match user_input_length {
            3 => {
                let result = user_input_sorter::for_three_length_input(&user_input);
                match result {
                    //1 means the request is for all staff
                    1 => match list_of_staff.len() {
                        0 => println!("there are currently no staffs"),
                        _ => println!("{list_of_staff:?}"),
                    },
                    //2 means the request is for staff a department
                    2 => {
                        //get the department value from user input
                        //   let user_input_vector:Vec<&str> = user_input.split_whitespace().collect();
                        let department = user_input_vector.get(2).unwrap();
                        //loop through the list of department and check if any department matches
                        // with the requested department
                        let does_department_match = user_input_sorter::check_if_department_match(
                            department,
                            &list_of_department,
                        );

                        match does_department_match {
                            true => {
                                let department_staffs =
                                    organisation_hashmap.get(department).unwrap();
                                println!("{department_staffs:?}");
                            }
                            _fasle => println!("sorry it doesn't match with any department"),
                        }
                    }
                    _ => println!("invalid input please input again"),
                }
            }
            4 => {
                let result = user_input_sorter::for_four_length_input(user_input.as_str());
                match result {
                    // 1 means add to department
                    1 => {
                        let person_name = user_input_vector.get(1).unwrap();
                        let department_name = user_input_vector.get(3).unwrap();
                        list_of_staff.push(person_name.clone());
                        list_of_department.push(department_name.clone());
                        let new_staff = organisation_hashmap
                            .entry(department_name.clone())
                            .or_insert(String::from(""));
                        new_staff.push(' ');
                        new_staff.push_str(person_name);

                        println!("{person_name} have to added to {department_name} department")
                    }
                    // 2 means remove from department
                    2 => {
                        // let user_input_vector:Vec<_> = input.split_whitespace().collect();
                        let person_name = user_input_vector.get(1).unwrap();
                        let department_name = user_input_vector.get(3).unwrap();

                        let does_department_match = user_input_sorter::check_if_department_match(
                            department_name,
                            &list_of_department,
                        );

                        match does_department_match {
                            true => {
                                let list_of_staffs_in_the_department =
                                    organisation_hashmap.get(department_name).unwrap();
                                let mut list_of_staffs_vec = Vec::new();
                                for word in list_of_staffs_in_the_department.split_whitespace() {
                                    list_of_staffs_vec.push(word);
                                }
                                // if any of the staff name match the name to be removed then. remove it
                                if let Some(staff) =
                                    list_of_staffs_vec.iter().position(|x| *x == person_name)
                                {
                                    list_of_staffs_vec.remove(staff);
                                    let edited_list_of_staffs = list_of_staffs_vec.join(" ");
                                    organisation_hashmap
                                        .insert(department_name.clone(), edited_list_of_staffs);

                                    if let Some(staff) =
                                        list_of_staff.iter().position(|x| x == person_name)
                                    {
                                        list_of_staff.remove(staff);
                                    }
                                }
                            }
                            false => println!("sorry there is no department with that name"),
                        };
                    }
                    _ => println!("invalid input please input again"),
                }
            }
            _ => (),
        }
    }
}
