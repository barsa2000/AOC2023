use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type Parsed = Vec<String>;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Parsed {
    input.lines().map(|l| l.trim().to_string()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &Parsed) -> usize {
    input
        .iter()
        .map(|l| {
            let f = l
                .chars()
                .find_map(|c| c.to_string().parse::<usize>().ok())
                .unwrap();
            let l = l
                .chars()
                .rev()
                .find_map(|c| c.to_string().parse::<usize>().ok())
                .unwrap();
            l + f * 10
        })
        .sum()
}

fn find(s: &str, r: &Regex) -> usize {
    let r = r.captures_iter(s).next().unwrap().get(1).unwrap().as_str();
    match r {
        "zero" | "0" => 0,
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0,
    }
}

#[aoc(day1, part2)]
fn part2(input: &Parsed) -> usize {
    let r_f = Regex::new(r"(?m)(one|two|three|four|five|six|seven|eight|nine|\d).*").unwrap();
    let r_l = Regex::new(r"(?m).*(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    input
        .iter()
        .map(|l| find(l, &r_f) * 10 + find(l, &r_l))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
    }
    fn input2<'a>() -> &'a str {
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 142);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input2())), 281);
    }
}
