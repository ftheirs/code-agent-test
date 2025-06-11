use binary_search::binary_search;

fn main() {
    let numbers = [2, 5, 7, 8, 11, 12];

    println!("Index of 7: {:?}", binary_search(&numbers, &7)); // Some(2)
    println!("Index of 13: {:?}", binary_search(&numbers, &13)); // None

    let strings = ["apple", "banana", "cherry"];
    println!("Index of \"banana\": {:?}", binary_search(&strings, &"banana")); // Some(1)
    println!("Index of \"grape\": {:?}", binary_search(&strings, &"grape")); // None
}
