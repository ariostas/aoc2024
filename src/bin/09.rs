/*!
# 2024 Day 9: Sample
##  Simple template

<https://adventofcode.com/2024/day/9>

This is a small example to get started, also functions as a template for new days.
*/

use std::collections::HashMap;

use aoc2024::run;

fn check(checksum: &mut i64, disk_pos: &mut i32, len: i32, fid: i32) {
    for _ in 0..len {
        *checksum += (fid as i64) * (*disk_pos as i64);
        *disk_pos += 1;
    }
}

fn solution_a(input: &str) -> i64 {
    let parsed_input = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();

    let mut checksum = 0;

    let mut left_pos = 0;
    let mut right_pos = input.len() as i32;
    let mut right_remaining = 0;

    let mut disk_pos = 0;

    while left_pos < right_pos {
        let mut len = parsed_input[left_pos as usize];
        if left_pos % 2 == 0 {
            check(&mut checksum, &mut disk_pos, len, left_pos / 2);
        } else {
            while len > 0 {
                if right_remaining == 0 {
                    right_pos -= 1;
                    if right_pos % 2 == 1 {
                        continue;
                    }
                    if right_pos < left_pos {
                        break;
                    }
                    right_remaining = parsed_input[right_pos as usize];
                }
                if right_remaining > len {
                    check(&mut checksum, &mut disk_pos, len, right_pos / 2);
                    right_remaining -= len;
                    len = 0;
                } else {
                    check(&mut checksum, &mut disk_pos, right_remaining, right_pos / 2);
                    len -= right_remaining;
                    right_remaining = 0;
                }
            }
        }
        left_pos += 1;
    }
    if left_pos == right_pos {
        check(&mut checksum, &mut disk_pos, right_remaining, right_pos / 2);
    }

    checksum
}

fn solution_b(input: &str) -> i64 {
    let parsed_input = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();

    let mut checksum = 0;

    let mut file_moved = vec![false; (parsed_input.len() + 1) / 2];
    let mut empty_lengths = parsed_input
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, c)| *c)
        .collect::<Vec<i32>>();
    let mut relocated_to_empty: HashMap<usize, Vec<(i32, i32)>> = HashMap::new();

    for right_pos in (0..(parsed_input.len() + 1) / 2).rev() {
        let len = parsed_input[right_pos * 2];
        for left_pos in 0..right_pos {
            if len <= empty_lengths[left_pos] {
                let relocated = relocated_to_empty.entry(left_pos).or_default();
                relocated.push((right_pos as i32, len));
                file_moved[right_pos] = true;
                empty_lengths[left_pos] -= len;
                break;
            }
        }
    }

    let mut disk_pos = 0;
    for left_pos in 0..parsed_input.len() {
        if left_pos % 2 == 0 {
            if file_moved[left_pos / 2] {
                disk_pos += parsed_input[left_pos];
                continue;
            }
            check(
                &mut checksum,
                &mut disk_pos,
                parsed_input[left_pos],
                (left_pos / 2) as i32,
            );
        } else {
            if let Some(relocated) = relocated_to_empty.get(&(left_pos / 2)) {
                for (right_pos, len) in relocated {
                    check(&mut checksum, &mut disk_pos, *len, *right_pos);
                }
            }
            disk_pos += empty_lengths[left_pos / 2];
        }
    }

    checksum
}

fn main() {
    run("09", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/09.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(solution_a(INPUT), 1928);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(solution_b(INPUT), 2858);
    }
}
