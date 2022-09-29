pub fn create_next_level(current_level: &Vec<String>) -> Vec<String> {
    let mut new_level: Vec<String> = Vec::new();
    let mut leave = String::from("");

    // taking two leaves from the current level and combining them
    for (index, value) in current_level.iter().enumerate() {
        //Check for missing pair of hashes
        if index % 2 == 0 {
            leave = value.clone();
        } else {
            leave.push_str(&value);
            new_level.push(leave.clone());
        }
    }

    new_level
}
