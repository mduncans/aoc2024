use anyhow;
use std::{fs, iter::zip, path::PathBuf};

fn read_input(input: PathBuf) -> anyhow::Result<String> {
    let contents = match fs::read_to_string(&input) {
        Ok(c) => c,
        Err(e) => anyhow::bail!("Failed to read input: {:?}. Error: {:?}", &input, e),
    };

    Ok(contents)
}

fn parse_contents(contents: &str) -> anyhow::Result<(Vec<i64>, Vec<i64>)> {
    let parsed_string_input: Vec<&str> = contents.split_whitespace().collect();

    let mut left_result: Vec<i64> = Vec::new();
    let mut right_result: Vec<i64> = Vec::new();

    for (index, string_num) in parsed_string_input.into_iter().enumerate() {
        if index % 2 == 0 {
            left_result.push(
                string_num
                    .parse()
                    .unwrap_or_else(|_| panic!("Failed to parse {:?} as i64", string_num)),
            );
        } else {
            right_result.push(
                string_num
                    .parse()
                    .unwrap_or_else(|_| panic!("Failed to parse {:?} as i64", string_num)),
            );
        }
    }

    Ok((left_result, right_result))
}

fn compute_total_distance(
    mut left_list: Vec<i64>,
    mut right_list: Vec<i64>,
) -> anyhow::Result<i64> {
    let mut result: i64 = 0;

    left_list.sort();
    right_list.sort();

    for (l, r) in zip(left_list, right_list) {
        result += (l - r).abs()
    }

    Ok(result)
}

pub fn solve(input: PathBuf) -> anyhow::Result<i64> {
    let contents = read_input(input).unwrap();
    let (lr, rr) = parse_contents(&contents).unwrap();
    let result = compute_total_distance(lr, rr)?;

    Ok(result)
}
