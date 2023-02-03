/*
YEW TUTORIAL APP by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

use rand::seq::SliceRandom;

/// Returns a vector of strings from a character split for a string.
/// Both the string and split character have to be strings.
pub fn clean_split(subject: &String, split_char: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in subject.split(split_char) {
        let new_item: String = item.to_string();
        result.push(new_item);
    }
    return result;
}

// Checks whether the user input is a valid
// integer.
pub fn is_int(subject: &String) -> bool {
    let mut result: bool = false;
    let try_parse = subject.parse::<i32>();
    match try_parse {
        Ok(_n) => {
            result = true;
        },
        Err(_e) => {
            // Do nothing.
        }
    };
    return result;
}

// Attempts to convert a string into an integer.
// Returns 0 if it cannot be done.
pub fn to_int(subject: &String) -> i32 {
    let mut result: i32 = 0;
    if is_int(subject) {
        result = subject.parse::<i32>().unwrap();
    }
    else {
        // Do nothing.
    }
    return result;
}

// Generates a password of given length.
pub fn make_pwd(length: &i32) -> String {
    let mut result_array: Vec<String> = Vec::new();
    let chars: String = String::from(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890_.;@*+"
    );
    let chars_list: Vec<String> = clean_split(&chars, &String::from(""));
    for _i in 0..*length {
        let rand_char: String = chars_list.choose(
            &mut rand::thread_rng()
        ).unwrap().to_string();
        result_array.push(rand_char);
    }
    let result: String = result_array.join("");
    return result;
}