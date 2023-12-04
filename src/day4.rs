#[inline(always)]
fn parse_card(numbers: &str) -> impl Iterator<Item = usize> + '_{
    numbers
        .split(" ")
        .filter_map(|x| {
            x.parse::<usize>().ok()
        })

}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> usize {
    input.lines().map(|line| {
        let card = 
        line.split_once(": ").unwrap().1
            .split_once(" | ").unwrap();
        
        let winning = parse_card(card.0);
        let yours:Vec<usize> = parse_card(card.1).collect();

        let wins = winning.filter(|w| {
            yours.iter().find(|&x| x == w).is_some()
        }).count();

        if wins == 0 {0} else {1 << (wins-1)}

    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT:&str = include_str!("../test_input/2023/day4.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(TEST_INPUT), 13);
    }
}