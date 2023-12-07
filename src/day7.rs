// Constants for determining if jokers are wild
const WILD:bool = true;
const NOT_WILD:bool = false;

// Converts a card to its numerical value; wild = 1
fn card_to_value<const JACKS_WILD:bool>(card: char) -> u8{
    match card {
        '2'..='9' => card.to_digit(10).unwrap() as u8,
        'T' => 10,
        'J' => if JACKS_WILD {1} else {11},
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unrecognised card: {card}")
    }
}

// Count occurences of the two most common cards in the array
fn count_hand(hand: &mut [u8;5]) -> u8{
    // Sort the hand so contiguous elements are together
    hand.sort_unstable();

    // Parse the hand to count the most common cards
    let mut last_card = 0;
    let mut ptr = 0;
    let mut wildcards = 0;

    // Reverse to ensure all wildcards are last
    for i in (0..5).rev() {
        // Count up the cards moving on if we see a new one
        if hand[i] == 1 { wildcards += 1}
        else if hand[i] != last_card{
            last_card = hand[i];
            ptr += 1;
            hand[ptr-1] = 1;
        } else {
            hand[ptr-1] += 1;
        }
    }

    // Sort the counts so the highest counts are at the end
    hand.sort_unstable();

    // Add any wildcards to the highest card
    hand[4] += wildcards;

    // Return the combined card counts
    (hand[4] << 4) + hand[3]
}

pub fn solve<const JACKS_WILD:bool>(input: &str) -> usize {

    // Array to re-use for storing sorted array
    let mut hand_buffer:[u8;5] = [0,0,0,0,0];


    let mut hands = input.lines().map(|line|{
        let (cards, bet) = line.split_once(' ').unwrap();
        cards.chars().enumerate().for_each(|(i,card)|{
            hand_buffer[i] = card_to_value::<JACKS_WILD>(card)
        });

        // We want to lexicographically order by [...count_hand, ...hand]
        // But to make this efficient we can combine the hand into one u32

        let card_order_score = hand_buffer.iter().fold(0, |acc, &card|{
            (acc << 4) + card as u32
        });

        let hand_score = count_hand(&mut hand_buffer);

        let score = ((hand_score as u32) << 20) + card_order_score;

        (score, bet.parse::<usize>().unwrap())
    }).collect::<Vec<(u32, usize)>>();

    hands.sort_by_key(|(score, _)| *score);

    hands.iter().enumerate().map(|(rank, (_score, bet))| {
        (rank+1) * bet
    }).sum()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> usize {
    solve::<NOT_WILD>(input)
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> usize {
    solve::<WILD>(input)
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT:&str = include_str!("../test_input/2023/day7.txt");
    const FULL_INPUT:&str = include_str!("../input/2023/day7.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(TEST_INPUT), 6440 + 628 * 6);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(TEST_INPUT), 5905 + 628 * 6);
    }

    #[test]
    fn full_input() {
        assert_eq!(solve_part1(FULL_INPUT), 246163188);
        assert_eq!(solve_part2(FULL_INPUT), 245794069);
    }

    #[test]
    fn count(){
        assert_eq!(count_hand(&mut [3,3,3,3,2]), 0x41);
        assert_eq!(count_hand(&mut [3,3,3,2,2]), 0x32);
        assert_eq!(count_hand(&mut [2,14,14,14,14]), 0x41);
        assert_eq!(count_hand(&mut [7,7,8,8,8]), 0x32);
        assert_eq!(count_hand(&mut [2,3,4,5,6]), 0x11);
    }
}