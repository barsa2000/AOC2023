use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = HashMap<usize, Vec<Vec<(Color, usize)>>>;

enum Color {
    R,
    G,
    B,
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(':').unwrap();
            (
                left.split_once(' ').unwrap().1.parse::<usize>().unwrap(),
                right
                    .split(';')
                    .map(|s| {
                        let split = s.split(',');
                        split
                            .map(|e| {
                                let (left, right) = e.trim().split_once(' ').unwrap();
                                (
                                    match right.trim() {
                                        "red" => Color::R,
                                        "green" => Color::G,
                                        "blue" => Color::B,
                                        _ => unreachable!(),
                                    },
                                    left.parse().unwrap(),
                                )
                            })
                            .collect::<Vec<(Color, usize)>>()
                    })
                    .collect(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Parsed) -> usize {
    input
        .iter()
        .filter_map(|(key, val)| {
            if val.iter().all(|gr| {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;
                for (c, n) in gr.iter() {
                    match c {
                        Color::R => r += n,
                        Color::G => g += n,
                        Color::B => b += n,
                    }
                }
                r <= 12 && g <= 13 && b <= 14
            }) {
                Some(key)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &Parsed) -> usize {
    input
        .iter()
        .map(|(_, val)| {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            val.iter().for_each(|gr| {
                for (c, n) in gr.iter() {
                    match c {
                        Color::R => r = r.max(*n),
                        Color::G => g = g.max(*n),
                        Color::B => b = b.max(*n),
                    }
                }
            });
            r * g * b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input1<'a>() -> &'a str {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    }
    fn input2<'a>() -> &'a str {
        input1()
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input1())), 8);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input2())), 2286);
    }
}
