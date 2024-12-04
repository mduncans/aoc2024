use crate::core::utils;
use anyhow;
use std::{iter::zip, path::PathBuf};

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

fn compute_similarity_score(left_list: Vec<i64>, right_list: Vec<i64>) -> anyhow::Result<i64> {
    let mut result: i64 = 0;

    for id in left_list.into_iter() {
        result += id * right_list.iter().filter(|&x| x == &id).count() as i64;
    }

    Ok(result)
}

pub fn solve(input: PathBuf, part: &str) -> anyhow::Result<i64> {
    let contents = utils::read_input(input).unwrap();
    let (lr, rr) = parse_contents(&contents).unwrap();

    match part {
        "a" => {
            let distance = compute_total_distance(lr, rr)?;
            Ok(distance)
        }
        "b" => {
            let similarity = compute_similarity_score(lr, rr)?;
            Ok(similarity)
        }
        _ => {
            let distance = compute_total_distance(lr, rr)?;
            Ok(distance)
        }
    }
}
