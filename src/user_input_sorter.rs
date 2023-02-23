pub fn for_three_length_input(value: &str) -> i32 {
    let lowercase_value: &String = &value.to_lowercase();
    let user_input_vector: Vec<&str> = lowercase_value.split_whitespace().collect();

    let result = {
        //check if the first word is list
        let check_for_list = matches!(*user_input_vector.first().unwrap(), "list");
        //check if the second word is of
        let check_for_of = matches!(*user_input_vector.get(1).unwrap(), "of");

        check_for_list && check_for_of
    };

    // 0 means invalid response from first two inputs
    let mut check_for_third_input = 0;

    //if the first two input where valid then, check for the third
    // if it is'nt valid then remain as zero
    if result {
        check_for_third_input = match *user_input_vector.get(2).unwrap() {
            "staffs" => 1,
            _ => 2,
        };
    };

    check_for_third_input
}

pub fn check_if_department_match(department: &String, list_of_department: &Vec<String>) -> bool {
    let mut result = false;
    for dept in list_of_department {
        if dept == department {
            result = true;
            break;
        }
    }
    result
}

pub fn for_four_length_input(value: &str) -> i32 {
    let lowercase_value: String = value.to_lowercase();
    let user_input_vector: Vec<&str> = lowercase_value.split_whitespace().collect();

    // checking for add and to to add a staff to a department
    let add_to_department = {
        //check if the first word is add
        let check_for_add = matches!(*user_input_vector.first().unwrap(), "add");
        //check if the third word is to
        let check_for_to = matches!(*user_input_vector.get(2).unwrap(), "to");

        check_for_add && check_for_to
    };

    // checking for "remove" and "from" to remove a staff from a department
    let remove_from_department = {
        //check if the first word is "remove"
        let check_for_remove = matches!(*user_input_vector.first().unwrap(), "remove");
        //check if the third word is "from"
        let check_for_from = matches!(*user_input_vector.get(2).unwrap(), "to");

        check_for_remove && check_for_from
    };

    // 0 is for invalid input
    let mut user_request = 0;
    //if add department and remove deparment are false remain at 0
    // if either of them is true, change user request value
    // 1 for "add" and 2 for "remove"
    if add_to_department {
        user_request = 1;
    };

    if remove_from_department {
        user_request = 2;
    };

    user_request
}

pub fn number_of_word_in_string(value: &str) -> usize {
    let lowercase_value = value.to_lowercase();
    let string_word_length: Vec<&str> = lowercase_value.split_whitespace().collect();
    string_word_length.len()
}
