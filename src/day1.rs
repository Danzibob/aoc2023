#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input.lines().map(|line|{
        let first_digit = line.chars().find_map(|x| x.to_digit(10));
        let last_digit  = line.chars().rev().find_map(|x| x.to_digit(10));
        first_digit.unwrap()*10 + last_digit.unwrap()
    }).sum()
}

const DIGITS:[&str;9] = ["one","two","three","four","five","six","seven","eight","nine"];

fn first_word(s: &str) -> Option<u32> {
    // Return early if a word can't fit in these characters
    if s.len() < 3 {return None}
    let mut number = 0;
    let mut position = usize::MAX;
    for (i, digit) in DIGITS.iter().enumerate(){
        if let Some(pos) = s.find(digit) {
            if pos < position {
                position = pos;
                number = i + 1;
                println!("new position: {}, new number: {}", position, number);
            }
        }
    }
    // If no number was found, return None
    if number == 0 {None} else {Some(number as u32)}
}

fn last_word(s: &str) -> Option<u32> {
    // Return early if a word can't fit in these characters
    if s.len() < 3 {return None}
    let mut number = 0;
    let mut position = 0;
    for (i, digit) in DIGITS.iter().enumerate(){
        if let Some(pos) = s.rfind(digit) {
            if pos >= position {
                position = pos;
                number = i + 1;
                println!("new position: {}, new number: {}", position, number);
            }
        }
    }
    // If no number was found, return None
    if number == 0 {None} else {Some(number as u32)}
}


#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> usize {
    // We know the inputs all have at least one digit due to part 1
    // So we can split the input by digits and only search the parts 
    // before and after the first and/or last digit
    input.lines().map(|line|{
        // Get the position and value of the first match
        let (d1_pos, chr1) = line.chars().enumerate()
                               .find(|(_i,x)| x.is_digit(10))
                               .unwrap();
        // and the position and value of the last match
        let (d2_pos, chr2) = line.chars().rev().enumerate()
                               .find(|(_i,x)| x.is_digit(10))
                               .unwrap();
        
        let d1 = first_word(&line[..d1_pos]).unwrap_or(
            chr1.to_digit(10).unwrap()
        );

        let d2 = last_word(&line[(line.len() - d2_pos - 1)..]).unwrap_or(
            chr2.to_digit(10).unwrap()
        );

        (d1*10 + d2) as usize
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1:&str = include_str!("../test_input/2023/day1-1.txt");
    const TEST_INPUT_2:&str = include_str!("../test_input/2023/day1-2.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(TEST_INPUT_1), 142);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(TEST_INPUT_2),281);
    }
}