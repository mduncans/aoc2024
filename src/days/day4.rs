use crate::core::utils;
use anyhow;
use std::path::PathBuf;

fn find_letter(rows: &Vec<&str>, letter: char) -> anyhow::Result<Vec<(usize, usize)>> {
    let mut results: Vec<(usize, usize)> = Vec::new();

    for (row_ind, &row) in rows.iter().enumerate() {
        for (col_ind, c) in row.chars().enumerate() {
            if c == letter {
                results.push((row_ind, col_ind))
            }
        }
    }

    Ok(results)
}

fn is_loc_next_letter(rows: &Vec<&str>, row_ind: usize, col_ind: usize, letter: char) -> bool {
    rows[row_ind].chars().nth(col_ind).unwrap() == letter
}

fn find_xmas(x_locations: &Vec<(usize, usize)>, rows: &Vec<&str>) -> anyhow::Result<i64> {
    let max_row_ind = rows.len() - 1;
    let max_col_ind = rows[0].len() - 1;

    let mut result = 0;

    for (row_ind, col_ind) in x_locations {
        // check right
        if col_ind <= &(max_col_ind - 3)
            && is_loc_next_letter(rows, *row_ind, *col_ind + 1, 'M')
            && is_loc_next_letter(rows, *row_ind, *col_ind + 2, 'A')
            && is_loc_next_letter(rows, *row_ind, *col_ind + 3, 'S')
        {
            result += 1;
        }

        // check left
        if col_ind >= &3
            && is_loc_next_letter(rows, *row_ind, *col_ind - 1, 'M')
            && is_loc_next_letter(rows, *row_ind, *col_ind - 2, 'A')
            && is_loc_next_letter(rows, *row_ind, *col_ind - 3, 'S')
        {
            result += 1;
        }

        // check down
        if row_ind <= &(max_row_ind - 3)
            && is_loc_next_letter(rows, *row_ind + 1, *col_ind, 'M')
            && is_loc_next_letter(rows, *row_ind + 2, *col_ind, 'A')
            && is_loc_next_letter(rows, *row_ind + 3, *col_ind, 'S')
        {
            result += 1;
        }

        // check up
        if row_ind >= &(3)
            && is_loc_next_letter(rows, *row_ind - 1, *col_ind, 'M')
            && is_loc_next_letter(rows, *row_ind - 2, *col_ind, 'A')
            && is_loc_next_letter(rows, *row_ind - 3, *col_ind, 'S')
        {
            result += 1;
        }

        // Check diagonals
        // check down right
        if col_ind <= &(max_col_ind - 3)
            && row_ind <= &(max_row_ind - 3)
            && is_loc_next_letter(rows, *row_ind + 1, *col_ind + 1, 'M')
            && is_loc_next_letter(rows, *row_ind + 2, *col_ind + 2, 'A')
            && is_loc_next_letter(rows, *row_ind + 3, *col_ind + 3, 'S')
        {
            result += 1;
        }

        // check down left
        if col_ind >= &3
            && row_ind <= &(max_row_ind - 3)
            && is_loc_next_letter(rows, *row_ind + 1, *col_ind - 1, 'M')
            && is_loc_next_letter(rows, *row_ind + 2, *col_ind - 2, 'A')
            && is_loc_next_letter(rows, *row_ind + 3, *col_ind - 3, 'S')
        {
            result += 1;
        }

        // check up right
        if col_ind <= &(max_col_ind - 3)
            && row_ind >= &(3)
            && is_loc_next_letter(rows, *row_ind - 1, *col_ind + 1, 'M')
            && is_loc_next_letter(rows, *row_ind - 2, *col_ind + 2, 'A')
            && is_loc_next_letter(rows, *row_ind - 3, *col_ind + 3, 'S')
        {
            result += 1;
        }

        // check up left
        if col_ind >= &3
            && row_ind >= &3
            && is_loc_next_letter(rows, *row_ind - 1, *col_ind - 1, 'M')
            && is_loc_next_letter(rows, *row_ind - 2, *col_ind - 2, 'A')
            && is_loc_next_letter(rows, *row_ind - 3, *col_ind - 3, 'S')
        {
            result += 1;
        }
    }

    Ok(result)
}

fn find_x_mas(m_locations: &Vec<(usize, usize)>, rows: &Vec<&str>) -> anyhow::Result<f64> {
    // M S   M M   S M   S S
    //  A     A     A     A
    // M S   S S   S M   M M
    // 1     2     3     4
    let max_row_ind = rows.len() - 1;
    let max_col_ind = rows[0].len() - 1;

    let mut result: f64 = 0.;

    // Previous solution of iterating through m locations
    // and counting will likely lead to over count. But
    // since there are two MAS per X, we can just add 0.5
    // per M. There are 4 types of X-mas to check for

    for (row_ind, col_ind) in m_locations {
        // Type 1 x-mas
        // top M
        if row_ind <= &(max_row_ind - 2)
            && col_ind <= &(max_col_ind - 2)
            && is_loc_next_letter(rows, *row_ind + 2, *col_ind, 'M')
            && is_loc_next_letter(rows, *row_ind, *col_ind + 2, 'S')
            && is_loc_next_letter(rows, *row_ind + 1, *col_ind + 1, 'A')
            && is_loc_next_letter(rows, *row_ind + 2, *col_ind + 2, 'S')
        {
            result += 0.5
        }
        // bottom M
        if row_ind >= &2
            && col_ind <= &(max_col_ind - 2)
            && is_loc_next_letter(rows, *row_ind - 2, *col_ind, 'M')
            && is_loc_next_letter(rows, *row_ind, *col_ind + 2, 'S')
            && is_loc_next_letter(rows, *row_ind - 1, *col_ind + 1, 'A')
            && is_loc_next_letter(rows, *row_ind - 2, *col_ind + 2, 'S')
        {
            result += 0.5
        }

        // Type 2 x-mas
        // left M
        if row_ind <= &(max_row_ind - 2)
            && col_ind <= &(max_col_ind - 2)
            && is_loc_next_letter(rows, *row_ind, *col_ind + 2, 'M')
            && is_loc_next_letter(rows, *row_ind + 2, *col_ind, 'S')
            && is_loc_next_letter(rows, *row_ind + 1, *col_ind + 1, 'A')
            && is_loc_next_letter(rows, *row_ind + 2, *col_ind + 2, 'S')
        {
            result += 0.5
        }
        //right M
        if row_ind <= &(max_row_ind - 2)
            && col_ind >= &2
            && is_loc_next_letter(rows, *row_ind, *col_ind - 2, 'M')
            && is_loc_next_letter(rows, *row_ind + 2, *col_ind, 'S')
            && is_loc_next_letter(rows, *row_ind + 1, *col_ind - 1, 'A')
            && is_loc_next_letter(rows, *row_ind + 2, *col_ind - 2, 'S')
        {
            result += 0.5
        }

        // Type 3 X-mas
        // top M
        if row_ind <= &(max_row_ind - 2)
            && col_ind >= &2
            && is_loc_next_letter(rows, *row_ind + 2, *col_ind, 'M')
            && is_loc_next_letter(rows, *row_ind, *col_ind - 2, 'S')
            && is_loc_next_letter(rows, *row_ind + 1, *col_ind - 1, 'A')
            && is_loc_next_letter(rows, *row_ind + 2, *col_ind - 2, 'S')
        {
            result += 0.5
        }
        // bottom M
        if row_ind >= &2
            && col_ind >= &2
            && is_loc_next_letter(rows, *row_ind - 2, *col_ind, 'M')
            && is_loc_next_letter(rows, *row_ind, *col_ind - 2, 'S')
            && is_loc_next_letter(rows, *row_ind - 1, *col_ind - 1, 'A')
            && is_loc_next_letter(rows, *row_ind - 2, *col_ind - 2, 'S')
        {
            result += 0.5
        }

        // Type 4 X-mas
        // left M
        if row_ind >= &2
            && col_ind <= &(max_col_ind - 2)
            && is_loc_next_letter(rows, *row_ind, col_ind + 2, 'M')
            && is_loc_next_letter(rows, *row_ind - 2, *col_ind, 'S')
            && is_loc_next_letter(rows, *row_ind - 1, *col_ind + 1, 'A')
            && is_loc_next_letter(rows, *row_ind - 2, *col_ind + 2, 'S')
        {
            result += 0.5
        }
        // right M
        if row_ind >= &2
            && col_ind >= &2
            && is_loc_next_letter(rows, *row_ind, col_ind - 2, 'M')
            && is_loc_next_letter(rows, *row_ind - 2, *col_ind, 'S')
            && is_loc_next_letter(rows, *row_ind - 1, *col_ind - 1, 'A')
            && is_loc_next_letter(rows, *row_ind - 2, *col_ind - 2, 'S')
        {
            result += 0.5
        }
    }
    Ok(result)
}

pub fn solve(input: PathBuf, part: &str) -> anyhow::Result<i64> {
    let contents = utils::read_input(input).unwrap();
    let rows: Vec<&str> = contents.split("\n").collect();

    match part {
        "a" => {
            let x_locations = find_letter(&rows, 'X').unwrap();
            let result = find_xmas(&x_locations, &rows).unwrap();
            Ok(result)
        }
        "b" => {
            let m_locations = find_letter(&rows, 'M').unwrap();
            let float_result = find_x_mas(&m_locations, &rows).unwrap();

            Ok(float_result.round() as i64)
        }
        _ => Ok(-1),
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day4::is_loc_next_letter;

    #[test]
    fn is_loc_next_letter_works() {
        let rows = vec!["..X...", ".SAMX.", ".A..A.", "XMAS.S", ".X...."];
        assert_eq!(is_loc_next_letter(&rows, 1, 3, 'M'), true);
    }
}
