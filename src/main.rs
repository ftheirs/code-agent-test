se generic_search::generic_search;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 7;
    match generic_search(&numbers, target) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found in the list", target),
    }

    let words = vec!["apple", "banana", "cherry", "date", "elderberry"];
    let target_word = "cherry";
    match generic_search(&words, target_word) {
        Some(index) => println!("Found \"{}\" at index {}", target_word, index),
        None => println!("\"{}\" not found in the list", target_word),
    }

    let empty_list: Vec<i32> = Vec::new();
    let target_empty = 5;
    match generic_search(&empty_list, target_empty) {
        Some(index) => println!("Found {} at index {}", target_empty, index),
        None => println!("{} not found in the empty list", target_empty),
    }

    let single_element_list = vec![42];
    let target_single_found = 42;
    match generic_search(&single_element_list, target_single_found) {
        Some(index) => println!("Found {} at index {}", target_single_found, index),
        None => println!("{} not found in the single element list", target_single_found),
    }

    let target_single_not_found = 100;
    match generic_search(&single_element_list, target_single_not_found) {
        Some(index) => println!("Found {} at index {}", target_single_not_found, index),
        None => println!("{} not found in the single element list", target_single_not_found),
    }
}
