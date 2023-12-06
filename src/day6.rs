use std::iter::zip;

fn win_race(t: i64, d:i64) -> usize{
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
    max - min + 1
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut lines = input.lines().map(|line|{
        line
            .split_whitespace().skip(1)
            .map(|number|{
                number.parse::<i64>().unwrap()
            })
    });
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    zip(times, distances).fold(1, |product, (t, d)|{
        product * win_race(t, d)
    })
}


#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut lines = input.lines().map(|line|{
        line
            .split_whitespace().skip(1)
            .collect::<String>()
            .parse::<i64>().unwrap()
    });
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();

    win_race(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT:&str = include_str!("../test_input/2023/day6.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(TEST_INPUT), 288);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(TEST_INPUT), 71503);
    }
}