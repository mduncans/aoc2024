use crate::core::read;
use anyhow;
use std::path::PathBuf;

fn convert_report_str_to_i64_vec(str_report: &str) -> anyhow::Result<Vec<i64>> {
    let result: Vec<i64> = str_report
        .split_whitespace()
        .map(|string_num| {
            string_num
                .parse()
                .unwrap_or_else(|_| panic!("Failed to parse {:?} as i64", string_num))
        })
        .collect();
    Ok(result)
}

fn is_monotonic(report: &Vec<i64>) -> bool {
    let monotonic_inc = report.windows(2).all(|w| w[0] < w[1]);
    let monotonic_dec: bool = report.windows(2).all(|w| w[0] > w[1]);

    if monotonic_dec | monotonic_inc {
        return true;
    } else {
        return false;
    }
}

fn is_safe(monotonic_report: Vec<i64>) -> bool {
    if !is_monotonic(&monotonic_report) {
        return false;
    }

    for window in monotonic_report.windows(2) {
        let distance = (window[0] - window[1]).abs();
        if distance > 3 || distance == 0 {
            return false;
        }
    }

    true
}

fn compute_safe_reports(str_reports: Vec<&str>) -> anyhow::Result<i64> {
    let result = str_reports
        .into_iter()
        .map(|str_report| {
            convert_report_str_to_i64_vec(str_report)
            .unwrap_or_else(|_| panic!("Could not convert str report: {:?} to Vec<i64>", str_report))
        })
        .map(|report| is_safe(report))
        .filter(|x| x == &true).count() as i64;

    Ok(result)
}

pub fn solve(input: PathBuf, part: &str) -> anyhow::Result<i64> {
    let contents = read::read_input(input).unwrap();
    let str_reports: Vec<&str> = contents.split("\n").collect();
    
    match part {
        _ => {
            let result = compute_safe_reports(str_reports).unwrap();
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day2::is_safe;

    #[test]
    fn test_simple_monotonic() {
        assert_eq!(is_safe(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(is_safe(vec![5, 4, 3, 2, 1]), true);
    }

    #[test]
    fn test_non_change() {
        assert_eq!(is_safe(vec![1, 2, 3, 3, 4]), false);
        assert_eq!(is_safe(vec![4, 3, 3, 2, 1]), false);
    }

    #[test]
    fn test_max_change() {
        assert_eq!(is_safe(vec![1, 4, 7, 10, 13]), true);
        assert_eq!(is_safe(vec![13, 10, 7, 4, 1]), true);
    }

    #[test]
    fn test_larger_change() {
        assert_eq!(is_safe(vec![1, 5, 6, 7, 8]), false);
        assert_eq!(is_safe(vec![13, 9, 8, 7, 6]), false);
    }

    #[test]
    fn example_tests() {
        assert_eq!(is_safe(vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_safe(vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_safe(vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_safe(vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_safe(vec![8, 6, 4, 4, 1]), false);
        assert_eq!(is_safe(vec![1, 3, 6, 7, 9]), true);
    }
}
