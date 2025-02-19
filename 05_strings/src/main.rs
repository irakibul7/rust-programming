fn main() {
    println!("\n===============================================");
    println!("🦀 Welcome to Strings! 🦀");
    println!("================================================\n");

    // Define a primitive String variable (&str)
    let language: &str = "Rust";
    println!("String literal: {}", language);
    println!("Length of the string literal: {}", language.len());

    // Create an empty String
    let course1 = String::new();

    // Convert String literal to String object using .to_string
    let s_course1 = course1.to_string();
    println!("This is an empty string: '{}'.", s_course1);
    println!("This is the length of my empty string: {}.", s_course1.len());

    // Create a String literal (&str)
    let course2: &str = "Rust Programming";

    // Convert &str to String
    let s_course2 = course2.to_string();
    println!("This is a String object: '{}'.", s_course2);
    println!("This is the length of my String object: {}.", s_course2.len());

    // Capacity in bytes
    println!("Capacity: {}.", s_course2.capacity());

    // Check if a string contains a substring
    println!(
        "Does '{}' contain '{}'? {}.",
        s_course2,
        language,
        s_course2.contains(language)
    );

    let replace_from = "Programming";
    let replace_to = "Language";

    // Replace a substring
    let result = s_course2.replace(replace_from, replace_to);
    println!("'{}' now becomes '{}'.", course2, result);

    // Trim whitespace
    let string = "   Rust     Programming     ".to_string();
    let trim_string = string.trim();
    println!("Trimmed string: '{}'", trim_string);

    // Split on whitespace
    println!("Splitting '{}' on whitespace:", course2);
    for token in course2.split_whitespace() {
        println!("'{}'", token);
    }

    // Split on token (comma)
    let str = String::from("Rust, Programming");
    println!("Splitting '{}' on comma:", str);
    for token in str.split(",") {
        println!("'{}'", token)
    }

    // Iterate over characters
    println!("Iterating over characters in '{}':", course2);
    for token in course2.chars() {
        println!("'{}'", token);
    }

    // Push a string
    let mut course = String::from("Rust");
    course.push_str(" Programming");
    println!("This is a beginner course in {}.", course);

    // Concatenation using + operator
    let course = "Rust".to_string();
    let course_type = " beginner course".to_string();
    let result = course + &course_type;
    println!("{}", result);

    let course = "Rust".to_string();
    let _course_type = "beginner course".to_string();

    // Default format macro
    let result = format!("{} {}", course, _course_type);
    println!("{}", result);
    // Passing value in the placeholder in the format macro
    let result = format!("{1} {0}", course, _course_type);
    println!("{}", result);

    // Slice
    let slice = &result[3..5];
    println!("Slice: '{}'", slice);

    // Calling Functions with String Types
    display_message(language);
    display_course(course);
}

fn display_message(my_course: &str) {
    println!("Course (String literal): {}", my_course)
}

fn display_course(my_course: String) {
    println!("Course (String object): {}", my_course);
}