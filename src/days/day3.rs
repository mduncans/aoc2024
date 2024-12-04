use crate::core::utils;
use anyhow;
use regex::Regex;
use std::path::PathBuf;

fn find_mul_instructions(memory: &str) -> anyhow::Result<Vec<&str>> {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let results: Vec<&str> = re
        .captures_iter(memory)
        .filter_map(|caps| caps.get(0).map(|m| m.as_str()))
        .collect();

    Ok(results)
}

fn find_mul_instructions_with_enablement(memory: &str) -> anyhow::Result<Vec<&str>> {
    let patterns = vec![r"do", r"don't", r"mul\(\d{1,3},\d{1,3}\)"];

    let mut matches = vec![];

    for pattern in patterns {
        let re = Regex::new(pattern).unwrap();
        for mat in re.find_iter(memory) {
            matches.push((mat.start(), mat.as_str()));
        }
    }

    // since do is found before don't we don't care that do is matched to don't
    matches.sort_by_key(|k| k.0);

    let result: Vec<&str> = matches.into_iter().map(|(_, mat)| mat).collect();

    Ok(result)
}

fn do_instructions(instruction: &str) -> anyhow::Result<i64> {
    // take index 4 to instruction.len() - 1 split at comma multiply return.
    let numbers_str = &instruction[4..instruction.len() - 1];
    let numbers = utils::str_to_veci64(numbers_str, ",").unwrap();

    let product: i64 = numbers.iter().product();

    Ok(product)
}

fn do_enabled_instructions(instructions: Vec<&str>) -> anyhow::Result<i64> {
    let mut enabled = true;
    let mut result: i64 = 0;

    for instruction in instructions {
        if instruction == "do" {
            enabled = true;
        } else if instruction == "don't" {
            enabled = false;
        } else if enabled {
            result += do_instructions(instruction).unwrap();
        }
    }

    Ok(result)
}

pub fn solve(input: PathBuf, part: &str) -> anyhow::Result<i64> {
    let contents = utils::read_input(input).unwrap();

    match part {
        "a" => {
            let instructions = find_mul_instructions(&contents).unwrap();
            let result = instructions
                .iter()
                .fold(0, |sum, &i| sum + do_instructions(i).unwrap());
            Ok(result)
        }
        "b" => {
            let instructions = find_mul_instructions_with_enablement(&contents).unwrap();
            let result = do_enabled_instructions(instructions).unwrap();
            Ok(result)
        }
        _ => {
            let result: i64 = 2;
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day3::find_mul_instructions;

    #[test]
    fn capture_3_digit_mul() {
        let empty_vec: Vec<&str> = Vec::new();
        assert_eq!(
            find_mul_instructions("mul(123,456)").unwrap(),
            vec!["mul(123,456)"]
        );
        assert_eq!(
            find_mul_instructions("mul(1,2)_mul(12,3),mul(1,23),mul(12,23)").unwrap(),
            vec!["mul(1,2)", "mul(12,3)", "mul(1,23)", "mul(12,23)"]
        );
        assert_eq!(find_mul_instructions("mul(1234,5678)").unwrap(), empty_vec);
    }
}
