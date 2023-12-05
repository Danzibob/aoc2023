#[derive(Clone, Copy)]
struct Mapping {
    src: usize,
    rng: usize,
    end: usize,
    offset: isize
}

impl std::fmt::Debug for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}-{} +{}]", self.src, self.end, self.offset)
    }
}

#[inline(always)]
fn parse_mapping(map: &str) -> Option<Mapping>{
    let mut numbers = [0; 3];
    let mut vals = map
        .split_whitespace()
        .take(4);

    // Take 3 numbers. If we run out, return None
    for i in 0..3 {
        if let Some(val) = vals.next()?.parse::<usize>().ok()
        { numbers[i] = val } else { return None }
    }

    let offset = numbers[0] as isize - numbers[1] as isize;

    Some(Mapping{
        src: numbers[1],
        rng: numbers[2],
        end: numbers[1] + numbers[2], // Redundant, but pre-computed here
        offset
    })
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut file_reader = input.lines();

    // Read seeds into a 
    let mut seeds:Vec<usize> = file_reader
        .next().unwrap()[7..]
        .split(" ").map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut grow = seeds.clone();

    file_reader.next(); // Consume empty line
    
    while file_reader.next().is_some() { // Consumes title
        while let Some(line) = file_reader.next(){

            // If the line is empty, move on to the next section
            if line == "" {
                // Propegate growth back to original vector
                seeds = grow.clone();
                continue;
            }

            // For every line with a valid mapping
            if let Some(map) = parse_mapping(line) {
                // Apply the mapping to all seeds
                for i in 0..(seeds.len()) {
                    if (seeds[i] >= map.src) && (seeds[i] < (map.src + map.rng)){
                        grow[i] = (seeds[i] as isize + map.offset) as usize;
                    }
                }
            }
        }
    }
    *grow.iter().min().unwrap()
}

use std::{cmp::{max, min}, process};

#[derive(Clone)]
struct SeedRange {
    start: usize,
    end: usize
}

impl std::fmt::Debug for SeedRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}-{})", self.start, self.end)
    }
}

fn map_range(range:&SeedRange, map:&Mapping) -> (Option<SeedRange>, Option<SeedRange>,Option<SeedRange>){
    // If the sections don't overlap, return original range
    if (range.start > map.end) || (range.end < map.src){
        return (None, None, None)
    }

    let overlap_start = max(map.src, range.start) as isize + map.offset;
    let overlap_end =   min(map.end, range.end) as isize + map.offset;

    (
        // Before the map
        if range.start < map.src
        {Some(SeedRange {start: range.start, end: map.src})}
        else {None},

        // Overlapping the map
        if overlap_start < overlap_end
        {Some(SeedRange {start: overlap_start as usize, end: overlap_end as usize})}
        else {None},

        // After the map
        if map.end < range.end
        {Some(SeedRange {start: map.end, end: range.end})}
        else {None}
    )
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut file_reader = input.lines();

    // Read seed values into a vec of ranges
    let mut seeds:Vec<SeedRange> = Vec::new();
    let mut seed_vals = file_reader
        .next().unwrap()[7..]
        .split(" ").map(|x| x.parse::<usize>().unwrap());
    while let Some(first_seed) = seed_vals.next(){
        seeds.push(SeedRange {
            start: first_seed,
            end: first_seed + seed_vals.next().unwrap()
        })
    }
    
    // Initialize an empty vec of ranges to grow into
    let mut maps: Vec<Vec<Mapping>> = Vec::new();
    let mut this_mapping: Vec<Mapping> = Vec::new();

    file_reader.next(); // Consume empty line
    
    while file_reader.next().is_some() { // Consumes title
        while let Some(line) = file_reader.next(){

            // If the line is empty, move on to the next section
            if line == "" {
                this_mapping.sort_unstable_by(|a,b| a.src.cmp(&b.src));
                maps.push(this_mapping);
                this_mapping = Vec::new();
                continue;
            }

            // if a line is valid then add to the mappings list
            if let Some(map) = parse_mapping(line) {
                this_mapping.push(map);
            }
        }
        maps.push(this_mapping.clone());
    }

    let mut grow:Vec<SeedRange> = Vec::new();

    let mut current_seed:SeedRange;

    for level in &maps{
        // println!("Starting next map level");
        for i in 0..(seeds.len()){
            current_seed = seeds[i].clone();
            // println!("Starting new seed {:?}", current_seed);
            let mut done = false;
            for map in level{
                match map_range(&current_seed, &map) {
                    // If there are no matches for this map, grow this seed and go to the next map
                    (None, None, None) => {},
                    (before, overlap, after) => {
                        // println!("{:?} {:?}", current_seed, map);
                        if let Some(new_seed) = before { 
                            // println!("B:{:?} ", new_seed);
                            grow.push(new_seed);
                        }
                        if let Some(new_seed) = overlap {
                            // println!("O:{:?} ", new_seed);
                            grow.push(new_seed);
                        }
                        if let Some(new_seed) = after {
                            // println!("A:{:?} ", new_seed);
                            // If there's still room to intersect more maps in this seed range
                            // Update the current seed and continue iterating over maps
                            current_seed.start = new_seed.start;
                            // println!("New Seed:{:?} ", current_seed);
                        } else {
                            done = true;
                            break;
                        }
                    }
                }
            }
            if !done { grow.push(current_seed.clone()) }
        }
        // Once all maps in this level have been matched,
        // Propegate the growth back to the seed vector
        seeds = grow.clone();
        grow.clear();
        // process::exit(0);
        // println!("All Seeds: {:?}\n", seeds);
    }
    seeds.iter().map(|range| range.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT:&str = include_str!("../test_input/2023/day5.txt");
    const FULL_INPUT:&str = include_str!("../input/2023/day5.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(TEST_INPUT), 35);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(TEST_INPUT), 46);
    }

    #[test]
    fn part1_full() {
        solve_part1(FULL_INPUT);
    }

    // Too low: 83692320
}