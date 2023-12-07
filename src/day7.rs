fn card_to_value(card: char) -> u8{
    match card {
        '2'..='9' => card.to_digit(10).unwrap() as u8,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unrecognised card: {card}")
    }
}

fn count_hand(hand: &mut [u8;5]) -> u16{
    hand.sort();
    
    let mut last_card = 0;
    let mut ptr = 0;
    let mut counts:[u8; 5] = [0,0,0,0,0];
    for &card in hand.iter() {
        if card != last_card{
            last_card = card;
            ptr += 1;
            if ptr == 5 {break}
        }
        counts[ptr] += 1;
    }

    counts.sort();

    ((counts[4] as u16) << 4) + counts[3] as u16
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> usize {

    // Array to re-use for storing sorted array
    let mut hand_buffer:[u8;5] = [0,0,0,0,0];


    let mut hands = input.lines().map(|line|{
        let (cards, bet) = line.split_once(" ").unwrap();
        cards.chars().enumerate().for_each(|(i,card)|{
            hand_buffer[i] = card_to_value(card)
        });

        // We want to lexicographically order by [...count_hand, ...hand]
        // But to make this efficient we can combine the hand into one u64

        let card_order_score = hand_buffer.iter().fold(0u64, |acc, &card|{
            (acc << 4) + card as u64
        });

        let hand_score = count_hand(&mut hand_buffer);

        let score = ((hand_score as u64) << 20) + card_order_score as u64;

        (score, bet.parse::<usize>().unwrap())
    }).collect::<Vec<(u64, usize)>>();

    hands.sort_by_key(|(score, _)| *score);

    hands.iter().enumerate().map(|(rank, (_score, bet))| {
        (rank+1) * bet
    }).sum()
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
    fn part1_full() {
        let score = solve_part1(FULL_INPUT);
        println!("Final Score: {}", score);
        assert!(245713384 < score);
    }

    #[test]
    fn count(){
        assert_eq!(count_hand(&mut [3,3,3,3,2]), 0x41);
        assert_eq!(count_hand(&mut [3,3,3,2,2]), 0x32);
        assert_eq!(count_hand(&mut [2,14,14,14,14]), 0x41);
        assert_eq!(count_hand(&mut [7,7,8,8,8]), 0x32);
        assert_eq!(count_hand(&mut [2,3,4,5,6]), 0x11);
    }

    // Too low: 245713384
}