se generic_search::generic_search;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 7;
    match generic_search(&numbers, target) {
        Some(index) => println!("Target {} found at index {}", target, index),
        None => println!("Target {} not found", target),
    }

    let words = vec!["apple", "banana", "cherry", "date", "elderberry"];
    let target_word = "cherry";
    match generic_search(&words, target_word) {
        Some(index) => println!("Target \"{}\" found at index {}", target_word, index),
        None => println!("Target \"{}\" not found", target_word),
    }

    let empty_list: Vec<i32> = Vec::new();
    let target_empty = 5;
    match generic_search(&empty_list, target_empty) {
        Some(index) => println!("Target {} found at index {}", target_empty, index),
        None => println!("Target {} not found in empty list", target_empty),
    }

    let single_element_list = vec![42];
    let target_single_found = 42;
    let target_single_not_found = 100;

    match generic_search(&single_element_list, target_single_found) {
        Some(index) => println!("Target {} found at index {}", target_single_found, index),
        None => println!("Target {} not found", target_single_found),
    }

    match generic_search(&single_element_list, target_single_not_found) {
        Some(index) => println!("Target {} found at index {}", target_single_not_found, index),
        None => println!("Target {} not found", target_single_not_found),
    }

    let boundary_list = vec![10, 20, 30, 40, 50];
    let target_first = 10;
    let target_last = 50;
    let target_middle = 30;

    match generic_search(&boundary_list, target_first) {
        Some(index) => println!("Target {} found at index {}", target_first, index),
        None => println!("Target {} not found", target_first),
    }

    match generic_search(&boundary_list, target_last) {
        Some(index) => println!("Target {} found at index {}", target_last, index),
        None => println!("Target {} not found", target_last),
    }

    match generic_search(&boundary_list, target_middle) {
        Some(index) => println!("Target {} found at index {}", target_middle, index),
        None => println!("Target {} not found", target_middle),
    }
}