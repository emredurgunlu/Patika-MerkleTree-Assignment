use super::create_next_level::create_next_level;
use super::hash_single_input::hash_single_input;
use std::fs;

pub fn merkle_root(filename: String) -> String {
    // Read Input Data from txt file
    let file_content: String = fs::read_to_string(&filename)
        .expect(format!("{} file should be included in this project", &filename).as_str());
    println!("{} file content:\n{}", filename, file_content);

    // Get vector of words which also contain tree_power
    let mut vec_leaves: Vec<String> = file_content
        .lines()
        .into_iter()
        .map(|s: &str| s.to_string())
        .collect();
    println!("vector of file content: {:?}", vec_leaves);

    let tree_power: u32 = vec_leaves[0].parse().unwrap();

    // Get rid of tree power in vector of words
    vec_leaves.remove(0);

    let mut hash_vec: Vec<String> = Vec::new();

    // Hash and calculate merklee tree
    for i in 0..(tree_power + 1) {
        hash_vec = vec_leaves
            .iter()
            .map(|i: &String| hash_single_input(i))
            .collect();
        vec_leaves = create_next_level(&hash_vec);
        println!("{}th leaves{:#?}", i, hash_vec);
    }

    hash_vec[0].to_string()
}
