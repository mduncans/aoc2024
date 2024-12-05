use crate::core::utils;
use std::path::PathBuf;

fn find_index(vec: &Vec<&str>, value: &str) -> anyhow::Result<usize> {
    vec.iter()
        .position(|&x| x == value)
        .ok_or_else(|| anyhow::anyhow!("Value '{}' not found in vector", value))
}

fn process_page_rules(page: &str, update: &Vec<&str>, rules: &Vec<&str>) -> bool {
    let mut result = true;

    let page_index = find_index(update, page).unwrap();

    for rule in rules {
        let rule_pages: Vec<&str> = rule.split("|").collect();
        if rule_pages[0] == page && update.contains(&rule_pages[1]) {
            let index2 = find_index(update, rule_pages[1]).unwrap();
            if page_index > index2 {
                result = false;
            }
        }
    }

    result
}

fn is_update_correct(update: &Vec<&str>, rules: &Vec<&str>) -> bool {
    let pages_with_rules: Vec<_> = rules.iter().flat_map(|r| r.split("|")).collect();

    let mut count = 0;

    for page in update {
        if pages_with_rules.contains(page) {
            if process_page_rules(page, update, rules) {
                count += 1
            }
        } else {
            count += 1
        }
    }

    count == update.len()
}

fn process_update(update: &str, rules: &Vec<&str>) -> anyhow::Result<i64> {
    let update_vec: Vec<&str> = update.split(',').collect();
    let mut result: i64 = 0;
    
    if is_update_correct(&update_vec, rules) {
        let mid_index = update_vec.len() / 2;
        result += update_vec[mid_index]
            .parse::<i64>()
            .unwrap_or_else(|_| panic!("Failed to parse {:?} as i64", update_vec[mid_index]));
    }

    Ok(result)
}

pub fn solve(input: PathBuf, part: &str) -> anyhow::Result<i64> {
    let contents = utils::read_input(input).unwrap();
    let rules_and_updates: Vec<&str> = contents.split("\n").collect();

    let (rules, updates) = utils::split_vector_at_empty(rules_and_updates);

    match part {
        "a" => {
            let result = updates
                .iter()
                .fold(0, |sum, &i| sum + process_update(i, &rules).unwrap());
            Ok(result)
        }
        _ => {
            let result: i64 = 2;
            Ok(result)
        }
    }
}
