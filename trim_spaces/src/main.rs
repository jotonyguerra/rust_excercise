// Trim whitespace from string without using the std::trim() function
fn main() {
    //println!("Hello, world!");

    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("    There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear.  ");
    assert_eq!(trim_spaces(&test3), "There's space to the rear.");

    let test4 = " We're surrounded by space!   ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = "🚀";
    assert_eq!(trim_spaces(test7), "🚀");

    println!("Test Passed!");
}

fn trim_spaces(s: &str) -> &str {
    let mut start_str = 0;
    for (index, char) in s.chars().enumerate() {
        if char != ' ' {
            start_str = index;
            break;
        }
    }

    let mut end = 0;
    for (index, char) in s.chars().rev().enumerate() {
        if char != ' ' {
            end = s.len() - index;
            break;
        }
    }

    return &s[start_str..end];
}