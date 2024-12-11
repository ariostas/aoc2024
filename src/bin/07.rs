/*!
# 2024 Day 7: Sample
##  Simple template

<https://adventofcode.com/2024/day/7>

This is a small example to get started, also functions as a template for new days.
*/

use aoc2024::run;

fn read_lines(input: &str) -> impl Iterator<Item = (i64, Vec<i64>)> + '_ {
    input.lines().filter_map(|line| {
        let mut sides = line.split(':');
        let test_value = sides.next()?.trim().parse::<i64>().ok()?;
        let list_of_numbers = sides
            .next()?
            .split_whitespace()
            .filter_map(|num| num.parse::<i64>().ok())
            .collect();
        Some((test_value, list_of_numbers))
    })
}

fn concat(a: i64, b: i64) -> i64 {
    let n_digits = b.ilog10() + 1;
    let pow10 = 10i64.pow(n_digits);
    a * pow10 + b
}

fn check_feasibility<const WITH_CONCAT: bool>(test_value: i64, values: &[i64]) -> bool {
    let min = values.iter().skip(1).fold(values[0], |acc, &x| {
        let mut r = acc + x;
        let prod = acc * x;
        if prod < r {
            r = prod;
        }
        if WITH_CONCAT {
            let concat = concat(acc, x);
            if concat < r {
                r = concat;
            }
        }
        r
    });
    if test_value < min {
        return false;
    }
    let max = values.iter().skip(1).fold(values[0], |acc, &x| {
        let mut r = acc + x;
        let prod = acc * x;
        if prod > r {
            r = prod;
        }
        if WITH_CONCAT {
            let concat = concat(acc, x);
            if concat > r {
                r = concat;
            }
        }
        r
    });
    if test_value > max {
        return false;
    }
    if test_value == min || test_value == max {
        return true;
    }
    if values.len() == 2 {
        return test_value == values[0] + values[1]
            || test_value == values[0] * values[1]
            || (WITH_CONCAT && test_value == concat(values[0], values[1]));
    }
    let last = values[values.len() - 1];
    let n_digits = last.ilog10() + 1;
    let pow10 = 10i64.pow(n_digits);
    return check_feasibility::<WITH_CONCAT>(test_value - last, &values[..values.len() - 1])
        || (test_value % last == 0
            && check_feasibility::<WITH_CONCAT>(test_value / last, &values[..values.len() - 1]))
        || (WITH_CONCAT
            && (test_value % pow10) == last
            && check_feasibility::<WITH_CONCAT>(test_value / pow10, &values[..values.len() - 1]));
}

fn solution_a(input: &str) -> i64 {
    let mut total_calibration = 0;
    for (test_value, numbers) in read_lines(input) {
        if check_feasibility::<false>(test_value, &numbers) {
            total_calibration += test_value;
        }
    }
    total_calibration
}

fn solution_b(input: &str) -> i64 {
    let mut total_calibration = 0;
    for (test_value, numbers) in read_lines(input) {
        if check_feasibility::<true>(test_value, &numbers) {
            total_calibration += test_value;
        }
    }
    total_calibration
}

fn main() {
    run("07", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/07.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(solution_a(INPUT), 3749);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(solution_b(INPUT), 11387);
    }
}
