use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};


fn create_priority_map(priority_string: String) -> HashMap<char, i32> {
    let mut priority_map: HashMap<char, i32> = HashMap::new();
    let mut priority = 1;
    for c in priority_string.chars() {
        priority_map.insert(c, priority);
        priority += 1;
    }
    priority_map
}

fn parse_data(priority_map: HashMap<char, i32>) -> Vec<Vec<i32>> {
    let filename = "src/rawdata.txt";
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut inv_vec: Vec<Vec<i32>> = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors because I'm lazy
        let data = line.chars();
        let mut current_vec: Vec<i32> = Vec::new();
        for char in data {
            current_vec.push(*priority_map.get(&char).unwrap());
        }
        inv_vec.push(current_vec);
    }

    return inv_vec;
}

fn calculate_shared_items(inventory: Vec<i32>) -> i32 {
    let length = inventory.len();
    let half_len = length / 2; // compare this to half_len-1 to see if something is in both lists
    for first_char in 0..half_len {
        for second_char in half_len..length {
            if inventory[first_char] == inventory[second_char] {
                return inventory[first_char];
            }
        }
    }
    return 0;
}

fn find_badge(inv_one: &Vec<i32>, inv_two: &Vec<i32>, inv_three: &Vec<i32>) -> i32 {
    let vec_one = inv_one;
    let vec_two = inv_two;
    let vec_three = inv_three;

    for item in vec_one {
        if vec_two.contains(&item) && vec_three.contains(&item) {
            return *item;
        }
    }
    return 0;
}

fn main() {
    let priority_map = create_priority_map("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string());
    let inv_vec = parse_data(priority_map);

    let mut total = 0;
    println!("inv_vec: {:?}", inv_vec);
    for inventory in inv_vec.clone() {
        total += calculate_shared_items(inventory);
    }
    println!("Total: {}", total);

    let mut badge_total = 0;
    for mut _inv_index in 0..inv_vec.clone().len()/3 {
        let local_ind = 3*_inv_index;
        badge_total += find_badge(&inv_vec[local_ind], &inv_vec[local_ind+1], &inv_vec[local_ind+2]);
    }
    println!("Badge Total: {}", badge_total);



}
