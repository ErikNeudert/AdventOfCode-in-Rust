use std::fs::File;
use std::io::{prelude::*, BufReader};
// use std::io::{BufReader};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let file = File::open("res/day5_1.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| match line {
        Ok(line) => line,
        Err(e) => panic!("Error reading line: {}", e)
    });
    let almanac: Almanac = parse_almanac(Box::new(lines));

    Ok(())
}

struct Almanac<'a> {
    seeds: Vec<usize>,
    maps: HashMap<(&'a str, &'a str), AlmanacMap<'a>>
}

struct AlmanacMap<'a> {
    from: &'a str,
    to: &'a str,
    conversion_ranges: Vec<AlmanacRange>
}

impl AlmanacMap<'_> {
    fn caluclate_destination(&self, source: usize) -> usize {
        //if not in any range, return source
        0
    }
}

#[derive(PartialEq, Eq, Debug)]
struct AlmanacRange {
    destination_range_start: usize, 
    source_range_start: usize,
    range_length: usize
}

impl AlmanacRange {
    fn new(destination_range_start: usize, source_range_start: usize, range_length: usize) -> Self {
        Self {
            destination_range_start: destination_range_start, 
            source_range_start: source_range_start,
            range_length: range_length
        }
    }

    fn caluclate_destination(&self, source: usize) -> Option<usize> {
        //if not in range, return None
        let source_range_end = self.source_range_start + self.range_length;
        if source >= self.source_range_start && source < source_range_end {
            let destination = source - self.source_range_start + self.destination_range_start;
            Some(destination)
        } else {
            None
        }
    }
}

//question:
//get soil nr for my seed:
//98 for 50, 99 for 51
//
// almanac, "seed", "fertilizer", 79);

//only works single, e.g.
fn find_destination_mapping(almanac: Almanac, from_to: (&str, &str), from_id: usize) -> usize { //better naming
    //50 98 2
    //recursive?

    // almanac
    0
}

//almanac is the 'newspaper' containing the crop and weather information for farmers
fn parse_almanac<'a>(mut lines: Box<dyn Iterator<Item=&'a str>>) -> Almanac<'a> {
    let seeds_line = lines.next().expect("'seeds' line required");
    let seeds: Vec<usize> = parse_seed_line(&seeds_line);
    assert_eq!("", lines.next().expect("expected empty line spacing"), "expected empty line");
    let maps: HashMap<(&str, &str), AlmanacMap> = parse_almanac_maps(lines);
    //first line should contain 

    Almanac {
        seeds: seeds,
        maps: maps
    }
}

fn parse_almanac_maps<'a>(lines: Box<dyn Iterator<Item=&'a str>>) -> HashMap<(&'a str, &'a str), AlmanacMap<'a>> {
    let mut res: HashMap<(&'a str, &'a str), AlmanacMap<'a>> = HashMap::new();
    //this shitty temp vec is required as the for loop takes ownership of the lines iter, 
    //and I can't just reuse the iter in the nested parse method parse_almanac_map (singular)
    let mut last_key: Option<(&str, &str)> = None;
    
    for line in lines {
        if line.ends_with("map:") {
            let (from, to) = parse_map_name(line);
            let new_map = AlmanacMap {
                from: from,
                to: to,
                conversion_ranges: vec![]
            };
            last_key = Some((from, to));
            res.insert((from, to), new_map);
        } else if line.is_empty() {
            //new map starts, clear last_key
            last_key = None;
        } else if line.starts_with(|c: char| c.is_numeric()) {
            //it's a range
            if let Some(from_to) = last_key {
                let range = parse_range(line);
                let map = match res.get_mut(&from_to) {
                    Some(map) => map,
                    None => panic!("Map for last_key missing, last_key: {:?}, line: {}", last_key, line)
                };
                map.conversion_ranges.push(range);
            } else {
                panic!("The last_key must be initialized, but isn't! line: {}", line);
            }
        } else {
            panic!("Line should end with 'map:' but was '{}'", line);
        }
    }

    return res;
}

fn parse_almanac_map<'a>(lines: Vec<&'a str>, (from, to): (&'a str, &'a str)) -> AlmanacMap<'a> {
    let mut conversion_ranges: Vec<AlmanacRange> = vec![];

    for line in lines {
        if line.is_empty() {
            break;
        }
        conversion_ranges.push(parse_range(line));
    }

    return AlmanacMap {
        from: from,
        to: to,
        conversion_ranges: conversion_ranges
    };
}

fn parse_range(line: &str) -> AlmanacRange {
    let split: Vec<usize> = line.split_whitespace()
        .into_iter()
        .map(|str| str.parse::<usize>().expect(&format!("Could not parse {str}")))//&
        .collect();
    return AlmanacRange::new(split[0], split[1], split[2]);
}

fn parse_map_name(line: &str) -> (&str, &str) {
    let split = line.strip_suffix("map:")
        .expect("line should be suffixed with 'map:'")
        .trim()
        .split_once("-to-")
        .expect("from and to should be separated by '-to-'");

    return (split.0, split.1);
}

fn parse_seed_line(line: &str) -> Vec<usize> {
    line.strip_prefix("seeds:").expect("Line should start with 'seeds:'")
        .split_whitespace()
        .into_iter()
        .map(|str| str.parse::<usize>().expect(&format!("Could not parse {str}")))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_find_destination_mapping_with_briding() {
        let input = "seeds: 79 14 55 13\n\
                     \n\
                     seed-to-soil map:\n\
                     50 98 2\n\
                     52 50 48\n\
                     \n\
                     soil-to-fertilizer map:\n\
                     0 15 37\n\
                     37 52 2\n\
                     39 0 15";
        let lines = Box::new(input.split("\n"));

        let almanac: Almanac = parse_almanac(lines);

        //find path from seed to fertilizer:
        //seed -> soil -> fertilizer
        //then create pairs, (seed, soil), (soil, fertilizer) and iterate them
        //maybe create a hashmap as result? e.g. from seed to fertilizer, seed->1, soild->5, fert->7 etc...

        //tests of the example
        let dest1 = find_destination_mapping(almanac, ("seed", "fertilizer"), 79);
        assert_eq!(81, dest1);
        let dest2 = find_destination_mapping(almanac, ("seed", "fertilizer"), 14);
        assert_eq!(53, dest2);
        let dest3 = find_destination_mapping(almanac, ("seed", "fertilizer"), 55);
        assert_eq!(57, dest3);
        let dest4 = find_destination_mapping(almanac, ("seed", "fertilizer"), 13);
        assert_eq!(52, dest4);
        
        //target: find location for each starting seed, and check which is the smalles
    }

    #[test]
    fn test_find_destination_mapping() {
        let input = "seeds: 79 14 55 13\n\
                     \n\
                     seed-to-soil map:\n\
                     50 98 2\n\
                     52 50 48";
        let lines = Box::new(input.split("\n"));

        let almanac: Almanac = parse_almanac(lines);
        // Seed number 79 corresponds to soil number 81.
        // Seed number 14 corresponds to soil number 14.
        // Seed number 55 corresponds to soil number 57.
        // Seed number 13 corresponds to soil number 13.
        
        //first example test
        let dest0 = find_destination_mapping(almanac, ("seed", "soil"), 50);
        assert_eq!(98, dest0);

        //tests of the example
        let dest1 = find_destination_mapping(almanac, ("seed", "soil"), 79);
        assert_eq!(81, dest1);
        let dest2 = find_destination_mapping(almanac, ("seed", "soil"), 14);
        assert_eq!(14, dest2);
        let dest3 = find_destination_mapping(almanac, ("seed", "soil"), 55);
        assert_eq!(57, dest3);
        let dest4 = find_destination_mapping(almanac, ("seed", "soil"), 13);
        assert_eq!(13, dest4);

    }

    #[test]
    fn test_parse_almanac() {
        let input = "seeds: 79 14 55 13\n\
                    \n\
                    seed-to-soil map:\n\
                    50 98 2\n\
                    52 50 48\n\
                    \n\
                    soil-to-fertilizer map:\n\
                    0 15 37\n\
                    37 52 2\n\
                    39 0 15";
        let lines = Box::new(input.split("\n"));
        
        let almanac: Almanac = parse_almanac(lines);
        assert_eq!(4, almanac.seeds.len());
        
        assert_eq!(79, almanac.seeds[0]);
        assert_eq!(14, almanac.seeds[1]);
        assert_eq!(55, almanac.seeds[2]);
        assert_eq!(13, almanac.seeds[3]);

        assert_eq!(2, almanac.maps.len());
        let map1: &AlmanacMap = almanac.maps.get(&("seed", "soil")).unwrap();
        assert_eq!("seed", map1.from);
        assert_eq!("soil", map1.to);
        assert_eq!(2, map1.conversion_ranges.len());
        assert_eq!(AlmanacRange::new(50, 98, 2), map1.conversion_ranges[0]);
        assert_eq!(AlmanacRange::new(52, 50, 48), map1.conversion_ranges[1]);
        
        let map2: &AlmanacMap = almanac.maps.get(&("soil", "fertilizer")).unwrap();
        assert_eq!("soil", map2.from);
        assert_eq!("fertilizer", map2.to);
        assert_eq!(3, map2.conversion_ranges.len());
        assert_eq!(AlmanacRange::new(0, 15, 37), map2.conversion_ranges[0]);
        assert_eq!(AlmanacRange::new(37, 52, 2), map2.conversion_ranges[1]);
        assert_eq!(AlmanacRange::new(39, 0, 15), map2.conversion_ranges[2]);
    }

    #[test]
    fn test_parse_almanac_map() {
        let map_ranges = "50 98 2\n\
                    52 50 48";
        let lines: Vec<&str> = map_ranges.split("\n")
            .collect();
        
        let map = parse_almanac_map(lines, ("seed", "soil"));

        assert_eq!("seed", map.from);
        assert_eq!("soil", map.to);

        assert_eq!(AlmanacRange::new(50, 98, 2), map.conversion_ranges[0]);
        assert_eq!(AlmanacRange::new(52, 50, 48), map.conversion_ranges[1]);
    }

    // #[test]
    // fn test_AlmanacRange() {
    //     let r1 = AlmanacRange::new(50, 98, 2);
    //     let r2 = AlmanacRange::new(52, 50, 48);

    //     //seed 98 => soil 50
    //     assert_eq!(50, r1.caluclate_destination(98));
    //     assert_eq!(51, r1.caluclate_destination(99));

    //     // 52 50 48
    //     // "The second line means that the source range starts at 50 and contains 48 
    //     // values: 50, 51, ..., 96, 97. This corresponds to a destination range 
    //     // starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed 
    //     // number 53 corresponds to soil number 55."
    //     assert_eq!(55, r2.caluclate_destination(53));
    //     assert_eq!(99, r2.caluclate_destination(97));
    //     assert_eq!(98, r2.caluclate_destination(98));
    // }

    #[test]
    fn test_AlmanacRange() {
        let r1 = AlmanacRange::new(50, 98, 2);
        let r2 = AlmanacRange::new(52, 50, 48);

        //seed 98 => soil 50
        assert_eq!(Some(50), r1.caluclate_destination(98));
        assert_eq!(Some(51), r1.caluclate_destination(99));
        assert_eq!(None, r1.caluclate_destination(97));

        // 52 50 48
        // "The second line means that the source range starts at 50 and contains 48 
        // values: 50, 51, ..., 96, 97. This corresponds to a destination range 
        // starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed 
        // number 53 corresponds to soil number 55."
        assert_eq!(Some(55), r2.caluclate_destination(53));
        assert_eq!(Some(99), r2.caluclate_destination(97));
        assert_eq!(None, r2.caluclate_destination(98));
        //None cases handled by the AlmanacMap
    }

    #[test]
    fn test_parse_map_name() {
        let line = "seed-to-soil map:";
        let (from, to) = parse_map_name(line);

        assert_eq!("seed", from);
        assert_eq!("soil", to);
    }

    #[test]
    fn test_parse_seed_line() {
        let line = "seeds: 79 14 55 13";
        let seeds: Vec<usize> = parse_seed_line(line);
        assert_eq!(4, seeds.len());
        
        assert_eq!(79, seeds[0]);
        assert_eq!(14, seeds[1]);
        assert_eq!(55, seeds[2]);
        assert_eq!(13, seeds[3]);
    }
}