use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct PartNum {
    value:usize,
    start:usize,
    end:usize
}

type Symbol = usize;

struct LineInfo {
    part_nums: Vec<PartNum>,
    symbols: Vec<Symbol>
}

lazy_static! {
    static ref NUMBER:regex::Regex = Regex::new(r"[0-9]+").unwrap();
    static ref SYMBOL:regex::Regex = Regex::new(r"[^0-9\.\n]").unwrap();
    static ref GEAR:regex::Regex = Regex::new(r"\*").unwrap();
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

        // Reuturn info about the whole line
        LineInfo { part_nums, symbols }

    }).collect::<Vec<LineInfo>>();

    let num_lines = input.lines().count();

    // Iterate through the collected data
    (0..(num_lines)).map(|i|{
        // Get the lines above and below the current one
        let above = if i>0 {lines_info.get(i-1)} else {None};
        let this = lines_info.get(i);
        let below = lines_info.get(i+1);
        
        // for each potential part number...
        this.unwrap().part_nums.iter().filter(|part|{
            // check if ANY line has a symbol
            [above, this, below].iter().any(|line|{
                if let Some(l) = line {
                    let left = if part.start == 0 {0} else {part.start-1};
                    let right = part.end + 1;
                    (&l.symbols).iter().any(|x| x >= &left && x < &right)
                } else {false}

            })
        // Add together the part numbers
        }).fold(0, |acc:usize, part:&PartNum| acc + part.value)
    }).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {

    // Collect info about each line
    let lines_info = input.lines().map(|line|{

        // Collect part number matches for the line
        NUMBER.captures_iter(line).map(|m|{
            let num =  m.get(0).unwrap();
            PartNum { 
                value: num.as_str().parse().unwrap(),
                start: num.start(),
                end: num.end()
            }
        }).collect()

    }).collect::<Vec<Vec<PartNum>>>();

    let num_lines = input.lines().count();

    const EMPTY:Vec<PartNum> = Vec::new(); // Gross hacky hack

    // Iterate through the collected data
    input.lines().enumerate().map(|(i, line)|{
        // Get above and below lines if they exist
        let above_ = if i > 0 {lines_info.get(i-1)} else {None};
        let below_ = if i < num_lines {lines_info.get(i+1)} else {None};
        let this = lines_info.get(i).unwrap();

        GEAR.captures_iter(line).map(|m|{
            let gear = m.get(0).unwrap();

            // Left of gear
            let left = this.iter().filter_map(|x| {
                if x.end == gear.start() 
                {Some(x.value)} else {None}
            }).collect::<Vec<usize>>();

            // Right of gear
            let right = this.iter().filter_map(|x| {
                if x.start == gear.end() 
                {Some(x.value)} else {None}
            }).collect();

            // Above gear
            let above = above_.unwrap_or(&EMPTY).iter().filter_map(|x| {
                if x.end >= gear.start() && x.start <= gear.end() 
                {Some(x.value)} else {None}
            }).collect();

            // Below gear
            let below = below_.unwrap_or(&EMPTY).iter().filter_map(|x| {
                if x.end >= gear.start() && x.start <= gear.end() 
                {Some(x.value)} else {None}
            }).collect();

            // Collect values together
            let values = [left,right,above,below].concat();

            // If there are exactly 2 values, return their product
            if values.len() == 2 
            { values.get(0).unwrap() * values.get(1).unwrap()}
            else { 0 }

        }).sum::<usize>()

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
        assert_eq!(solve_part1(CUSTOM_INPUT), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(TEST_INPUT), 467835);
    }
}