/*!
# 2024 Day 6: Guard Gallivant
## Walking with obstacles

<https://adventofcode.com/2024/day/6>

I'm using the direction code from my 2023 solutions. The first solution was
really slow, from using `HashMap`. Switching to the bitflag-like solution sped
it up, but it was still slow due to rerunning iterators. Using a little memory
is okay! Finally it was quite fast. The current solution adds parallel
processing for part 2, which works really well. I've simplifed the 2023
Direction by removing Position, which now only helps very slightly, due to
`.get` being smarter in Grid this year.
*/

use aoc2024::{run, Problem};

use aoc2024::grid::{read_char, Direction};
use grid::Grid;
use rayon::prelude::*;

enum Result {
    Exited(Grid<u8>),
    Cyclic,
}

fn next_step(map: &Grid<char>, pos: (i64, i64), dir: Direction) -> Direction {
    let next = pos + dir;
    if map.get(next.0, next.1) == Some(&'#') {
        next_step(map, pos, dir.clockwise())
    } else {
        dir
    }
}

fn get_pos(map: &Grid<char>) -> (i64, i64) {
    map.indexed_iter()
        .find_map(|((x, y), c)| {
            if *c == '^' {
                Some((x.try_into().unwrap(), y.try_into().unwrap()))
            } else {
                None
            }
        })
        .unwrap()
}

fn solve(map: &Grid<char>) -> Result {
    let mut pos = get_pos(map);
    let mut visited: Grid<u8> = Grid::new(map.rows(), map.cols());
    let mut dir = Direction::Up;
    while map.get(pos.0, pos.1).is_some() {
        visited[(pos.0.try_into().unwrap(), pos.1.try_into().unwrap())] |= dir as u8;
        dir = next_step(map, pos, dir);
        pos = pos + dir;
        if let Some(&v) = visited.get(pos.0, pos.1) {
            if v & dir as u8 != 0 {
                return Result::Cyclic;
            }
        }
    }
    Result::Exited(visited)
}

struct Day06 {}

impl Problem for Day06 {
    fn solution_a(input: &str) -> i64 {
        let map = read_char(input);
        if let Result::Exited(visited) = solve(&map) {
            visited
                .iter()
                .filter(|&v| *v > 0)
                .count()
                .try_into()
                .unwrap()
        } else {
            panic!("No solution found");
        }
    }

    fn solution_b(input: &str) -> i64 {
        let orig_map = read_char(input);
        orig_map
            .indexed_iter()
            .filter(|(_, &c)| c == '.')
            .collect::<Vec<_>>()
            .par_iter()
            .filter(|((x, y), _)| {
                let mut map = orig_map.clone();
                map[(*x, *y)] = '#';
                let result = solve(&map);
                matches!(result, Result::Cyclic)
            })
            .count()
            .try_into()
            .unwrap()
    }
}

fn main() {
    run::<Day06>("06");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../samples/06.txt");

    #[test]
    fn test_sample_a() {
        assert_eq!(Day06::solution_a(INPUT), 41);
    }

    #[test]
    fn test_sample_b() {
        assert_eq!(Day06::solution_b(INPUT), 6);
    }
}
