use regex::Regex;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input.lines().map(|line|{
        let first_digit = line.chars().find_map(|x| x.to_digit(10));
        let last_digit  = line.chars().rev().find_map(|x| x.to_digit(10));
        first_digit.unwrap()*10 + last_digit.unwrap()
    }).sum()
}

fn word_to_val(s:&str) -> u32{
    match s {
        "one" => 1, "two" => 2, "three" => 3,
        "four" => 4, "five" => 5, "six" => 6,
        "seven" => 7, "eight" => 8, "nine" => 9,
        _ => 0
    }
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let words = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();

    input.lines().map(|line|{

        // Get the position and value of the first match
        let (d1_pos, chr1) = line.chars().enumerate()
                               .find(|(_i,x)| x.is_digit(10))
                               .unwrap();
        // and the position and value of the last match
        let (d2_pos, chr2) = line.chars().rev().enumerate()
                               .find(|(_i,x)| x.is_digit(10))
                               .unwrap();

        // First digit
        let d1 = if let Some(word) = words.find(&line[..d1_pos])
        { word_to_val(word.as_str()) } else { chr1.to_digit(10).unwrap() };

        // Second digit
        let last_digit = line.len() - d2_pos - 1;
        let d2 = if let Some(word) = words.find_iter(&line[last_digit..]).last()
        { word_to_val(word.as_str()) } else { chr2.to_digit(10).unwrap() };
        
        d1*10 + d2

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