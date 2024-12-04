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
