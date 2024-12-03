use crate::core::read;
use anyhow;
use regex::Regex;
use std::path::PathBuf;

fn find_mul_instructions(instruction: &str) -> anyhow::Result<Vec<&str>> {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let results: Vec<&str> = re
        .captures_iter(instruction)
        .filter_map(|caps| caps.get(0).map(|m| m.as_str()))
        .collect();

    Ok(results)
}

fn do_instructions(instruction: &str) -> anyhow::Result<i64> {
    // take index 4 to instruction.len() - 1 split at comma multiply return.
    let numbers_str = &instruction[4..instruction.len() - 1];
    let numbers: Vec<i64> = numbers_str
        .split(",")
        .map(|string_num| {
            string_num
                .parse()
                .unwrap_or_else(|_| panic!("Failed to parse {:?} as i64", string_num))
        })
        .collect();

    let product: i64 = numbers.iter().product();

    Ok(product)
}

pub fn solve(input: PathBuf, part: &str) -> anyhow::Result<i64> {
    let contents = read::read_input(input).unwrap();
    let instructions = find_mul_instructions(&contents).unwrap();

    match part {
        "a" => {
            let result = instructions
                .iter()
                .fold(0, |sum, &i| sum + do_instructions(i).unwrap());
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
