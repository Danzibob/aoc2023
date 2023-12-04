#[inline(always)]
fn parse_card(numbers: &str) -> impl Iterator<Item = usize> + '_{
    numbers
        .split(" ")
        .filter_map(|x| {
            x.parse::<usize>().ok()
        })

}

fn count_wins(cards: &str) -> impl Iterator<Item = usize> + '_ {
    let mut yours:Vec<usize> = vec![0; 100]; // Arbitrary big number
    cards.lines().map(move |line| {
        let card = 
        line.split_once(": ").unwrap().1
            .split_once(" | ").unwrap();
        
        let winning = parse_card(card.0);
        yours.clear();
        yours.extend(parse_card(card.1));

        winning.filter(|w| {
            yours.iter().find(|&x| x == w).is_some()
        }).count()
    })
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> usize {
    count_wins(input).map(|wins| 
        if wins == 0 {0} else {1 << (wins-1)}
    ).sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> usize {
    let wins = count_wins(input);
    let mut num_cards = vec![0; 300]; // Arbitrary big number
    for (i,card_wins) in wins.enumerate() {
        num_cards[i] += 1; // Original card

        // Bonus card for each of n wins
        // x number of this card
        (1..(card_wins+1)).for_each(|bonus_card|{
            num_cards[i + bonus_card] += num_cards[i];
        });
    }

    num_cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT:&str = include_str!("../test_input/2023/day4.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(TEST_INPUT), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(TEST_INPUT), 30);
    }
}
