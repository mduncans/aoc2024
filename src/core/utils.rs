use std::{fs, path::PathBuf};

pub fn read_input(input: PathBuf) -> anyhow::Result<String> {
    let contents = match fs::read_to_string(&input) {
        Ok(c) => c,
        Err(e) => anyhow::bail!("Failed to read input: {:?}. Error: {:?}", &input, e),
    };

    Ok(contents)
}

pub fn str_to_veci64(numbers_str: &str, split: &str) -> anyhow::Result<Vec<i64>> {
    if split == "WHITESPACE" {
        let numbers: Vec<i64> = numbers_str
            .split_whitespace()
            .map(|string_num| {
                string_num
                    .parse()
                    .unwrap_or_else(|_| panic!("Failed to parse {:?} as i64", string_num))
            })
            .collect();
        Ok(numbers)
    } else {
        let numbers: Vec<i64> = numbers_str
            .split(split)
            .map(|string_num| {
                string_num
                    .parse()
                    .unwrap_or_else(|_| panic!("Failed to parse {:?} as i64", string_num))
            })
            .collect();
        Ok(numbers)
    }
}

pub fn split_vector_at_empty(vec: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let split_pos = vec.iter().position(|&x| x.is_empty()).unwrap_or(vec.len());
    let first_half = vec[..split_pos].to_vec();
    let second_half = vec[split_pos + 1..].to_vec(); // Skip the empty string
    (first_half, second_half)
}