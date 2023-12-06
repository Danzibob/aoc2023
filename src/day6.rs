use std::iter::zip;

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut lines = input.lines().map(|line|{
        line.split_whitespace().skip(1).map(|number|{
            number.parse::<isize>().unwrap()
        })
    });
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    let mut product = 1;

    for (t, d) in zip(times, distances){
        // distance achieved in the race is s(t-s)
        // so to win: s(t-s) - d > 0
        // first, solve -s^2 + ts - d = 0 to get bounds

        let dis = ((t*t - 4*d) as f64).sqrt();
        let x1 = (t as f64 - dis) / 2.0;
        let x2 = (t as f64 + dis) / 2.0;

        // We have to win, not just equal zero
        let min = if x1%1.0 == 0.0 {x1 + 1.0} else {x1.ceil()} as usize;
        let max = if x2%1.0 == 0.0 {x2 - 1.0} else {x2.floor()} as usize;

        // Add 1 because bounds are inclusive
        product *= max - min + 1;
    }

    product
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT:&str = include_str!("../test_input/2023/day6.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(TEST_INPUT), 288);
    }
}