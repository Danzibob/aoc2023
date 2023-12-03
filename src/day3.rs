use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct PartNum {
    value:usize, start:usize, end:usize
}

type Symbol = usize;

struct LineInfo {
    part_nums: Vec<PartNum>, symbols: Vec<Symbol>
}

lazy_static! {
    static ref NUMBER:regex::Regex = Regex::new(r"[0-9]+").unwrap();
    static ref SYMBOL:regex::Regex = Regex::new(r"[^0-9\.\n]").unwrap();
}

#[inline(always)]
fn search_in_range(s:&Vec<usize>, left:usize, right:usize) -> bool {
    let low = s.partition_point(|x| x < &left);
    let high = s.partition_point(|x| x < &right);
    return low < high
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {

    // Collect info about each line
    let lines_info = input.lines().map(|line|{

        // Collect part number matches for the line
        let part_nums = NUMBER.captures_iter(line).map(|m|{
            let num =  m.get(0).unwrap();
            PartNum { 
                value: num.as_str().parse().unwrap(),
                start: num.start(),
                end: num.end()
            }
        }).collect();

        // Collect symbol matches for the line
        let symbols = SYMBOL.captures_iter(line).map(|m|{
            let sym =  m.get(0).unwrap();
            sym.start() as Symbol
        }).collect();

        LineInfo { part_nums, symbols}
    }).collect::<Vec<LineInfo>>();

    let num_lines = input.lines().count();

    // Iterate through the collected data
    (0..(num_lines)).map(|i|{
        // Get the lines above and below the current one
        let above = if i>0 {lines_info.get(i-1)} else {None};
        let this = lines_info.get(i);
        let below = lines_info.get(i+1);
        
        // for each numbers, check if any line has a symbol
        this.unwrap().part_nums.iter().filter(|part|{
            [above, this, below].iter().any(|line|{
                match line {
                    Some(l) => search_in_range(&l.symbols, if part.start == 0 {0} else {part.start-1}, part.end+1),
                    None => false
                }
            })
        // Add together the part numbers
        }).fold(0usize, |acc:usize, part:&PartNum| acc + part.value)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT:&str = include_str!("../test_input/2023/day3.txt");
    const CUSTOM_INPUT:&str = include_str!("../test_input/2023/day3-custom.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(TEST_INPUT), 4361);
        println!("---------------");
        solve_part1(CUSTOM_INPUT);
    }

    #[test]
    fn search() {
        assert_eq!(search_in_range(&vec![1,2,3,4,5,6,7], 3,5), true);
        assert_eq!(search_in_range(&vec![1,2,3,5,6,7], 3,4), true);
        assert_eq!(search_in_range(&vec![1,2,4,5,6,7], 3,4), false);
        assert_eq!(search_in_range(&vec![1,2,6,7], 3,5), false);
    }
}