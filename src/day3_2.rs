use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::BTreeMap;
use std::ops::Bound::Included;
use std::iter::Peekable;
use std::io::{self};
use std::iter::Enumerate;
use std::str::Chars;
use std::fmt;
use std::cmp;

fn main() -> std::io::Result<()> {
    // The missing part wasn't the only issue - one of the gears in the engine is wrong. 
    // A gear is any * symbol that is adjacent to exactly two part numbers. 
    // Its gear ratio is the result of multiplying those two numbers together.

    // This time, you need to find the gear ratio of every gear and add them all up 
    // so that the engineer can figure out which gear needs to be replaced.

    // Consider the same engine schematic again:

    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..
    // In this schematic, there are two gears. 
    // The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. 
    // The second gear is in the lower right; its gear ratio is 451490. 
    // (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) 
    // Adding up all of the gear ratios produces 467835.

    let file = File::open("res/day3_1.txt")?;
    let reader = BufReader::new(file);
    let mut grid = Grid::new();

    fill_map_from_text(Box::new(reader.lines()), &mut grid)?;
    let gear_ratio_sum = sum_gear_ratios(grid);

    println!("Sum of gear ratios: {}", gear_ratio_sum);
    Ok(())
}

fn sum_gear_ratios(grid: Grid) -> i32 {
    let mut sum = 0;
    //for each gear token 
    let tokens = grid.tokens.iter().filter(|(_, token)| token.token_type == TokenType::Gear);
    for (point, token) in tokens {
        println!("{} {}", point, token);
        if token.value.len() != 1 {
            panic!("Gear should always be '*', but was: {}", token.value);
        }

        let surrounding_nums: Vec<i32> = grid.find_surroundings(point, token)
            .into_iter()
            .map(|(p, t)| t.value.parse::<i32>()
                .expect(&["Could not parse token value: ", &t.value].join(" ")))
            .collect();
        if surrounding_nums.len() > 1 {
            //it's a gear! sum it!
            let num_sum = surrounding_nums.iter().fold(1, |acc, num| acc * num);
            println!("surroundings summed: {}", num_sum);
            sum += num_sum;
        }

    }
    sum
}

//turns out it would have been way easier to just have a 2d Array and iterate that.
//no structs or anything, just recognizing all surroundings by walking around a point.
fn get_in_range<'a>(map: &'a BTreeMap<Point, Token>, range: &'a (Point, Point), token_type: TokenType) -> Vec<(&'a Point, &'a Token)> {
    let surroundings: Vec<(&Point, &Token)> = map.range((Included(range.0), Included(range.1)))
        .filter(|(_, token)| token.token_type == token_type)
        //I'd prefer to solve this by adding a Grid data type that handles the range checking,
        //but that's much more memory expensive, and the map.range selection with filtering is a good balance
        .filter(|(point, _)| (
            point.x >= range.0.x 
            && point.x <= range.1.x 
            && point.y >= range.0.y
            && point.y <= range.1.y
        ))
        .collect();
    surroundings
}

// fn fill_map_from_text(reader: BufReader<File>, grid: &mut Grid) -> Result<(), io::Error> {
fn fill_map_from_text(iterator: Box<dyn Iterator<Item=Result<String, std::io::Error>>>, grid: &mut Grid) -> Result<(), io::Error> {
    for (y, line) in iterator.enumerate() {
        let line: String = match line {
            Ok(line) => line,
            Err(e) => panic!("Error reading line {}", e)
        };
        let mut char_iter: Peekable<Enumerate<Chars>> = line.chars().enumerate().peekable();
        
        while let Some((x, char)) = char_iter.peek() {
            let x = *x;
            let char = *char;
            if char == '.' {
                //skip dots
                char_iter.next(); 
                grid.put_char(y, x, char);
                continue;
            }

            let token_type = TokenType::from(char);
            let mut token = Token {
                token_type: token_type,
                value: String::new()
            };
            while let Some((x, char)) = char_iter.peek() {
                let char = *char;
                if TokenType::from(char) == token.token_type {
                    let (x, next_char) = char_iter.next().expect("next should be present due to peek returning Some");
                    assert_eq!(char, next_char, "next should = peek");
                    grid.put_char(y, x, char);
                    token.value.push(char);
                } else {
                    break;
                }
            }
            grid.put_token(y, x, token);
        }
    }
    Ok(())
}

struct Grid {
    internal_map: Vec<Vec<char>>,
    tokens: BTreeMap<Point, Token>
}
impl Grid {
    fn put_char(&mut self, y: usize, x: usize, char: char) {
        // println!("{} {} {}", y, x, char);
        if self.internal_map.len() <= y {
            //need to resize
            self.internal_map.resize(y+1, vec![' '])
        }
        self.internal_map[y].insert(x, char);
    }
    fn put_token(&mut self, y: usize, x: usize, token: Token) {
        let point = Point::new(y, x);
        if let Some(old_value) = &self.tokens.insert(point, token) {
            panic!("Duplicate value in map, should not happen! {:?}", old_value);
        }
    }

    fn new() -> Self {
        Grid {internal_map: vec![vec![' ']], tokens: BTreeMap::new()}
    }

    fn find_surroundings(&self, point: &Point, token: &Token) -> Vec<(Point, &Token)> {
        let mut result: Vec<(Point, &Token)> = vec![];
        //calculate the from and to Points:
        let token_len = token.value.len();
        let (from_top_left, to_bottom_right) = point.surrounding_range(token_len);
        //scan the area for non '.'
        println!("surrounding area points: {} {}", from_top_left, to_bottom_right);

        for y in from_top_left.y..=to_bottom_right.y {
            let mut skip_chars = 0;
            for x in from_top_left.x..=to_bottom_right.x {
                if skip_chars > 0 {
                    skip_chars -= 1;
                    continue;
                }
                println!("{} {}", y, x);
                //skip the searched tokens range (hope i can just x += len :-)
                if point.x == x && point.y == y {
                    //skip the length of the token, to not double track it
                    skip_chars += token_len;
                    continue;
                }
                let char = *&self.internal_map[y][x];
                if char == '.' {
                    //skip dots
                    continue;
                }
                //we found a non '.', find the start index of the token
                let char_pos = Point::new(y, x);
                let token_start: Point = self.find_token_start(char_pos);
                if let Some(token) = self.tokens.get(&token_start) {
                    //skip the token, e.g. all chars from start to end
                    let token_end_x = token_start.x + token.value.len();
                    skip_chars += token_end_x - x;
                    result.push((token_start, token));
                } else {
                    panic!("Could not find a token that should be there!\n\
                              searching adjacents to token: {}, pos: {} \n. \
                              found adjacent char {}, pos: {},\n. \
                              calculated token start: {}", 
                              token, point,
                              char, char_pos,
                              token_start
                            );
                }
            }
        }
        result
    }

    fn find_token_start(&self, current_idx: Point) -> Point {
        //just walk leftwards from the start index, until you find a non matching token char
        let mut token_start_x: usize = current_idx.x;
        let token_type = TokenType::from(self.internal_map[current_idx.y][current_idx.x]);
        loop {
            if token_start_x == 0 {
                break; //break due to @start of line
            }
            let previous_char = self.internal_map[current_idx.y][token_start_x - 1];
            if token_type != TokenType::from(previous_char) {
                break; //found not matching token
            }
            token_start_x -= 1;
        }

        Point::new(current_idx.y, token_start_x)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    Numeric,
    Symbol,
    Dot,
    Gear //denoted by a '*'
    //it's actually not a gear, but only a maybe gear, as a Gear would be surrounded by two Numerics.
    //for easier impl we just assume every * is a gear, but only between two Numerics it's a useful gear
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Numeric => write!(f, "Numeric"),
            TokenType::Symbol  => write!(f, "Symbol"),
            TokenType::Gear  => write!(f, "Gear"),
            TokenType::Dot  => write!(f, "Dot")
        }
    }
}
impl From<char> for TokenType {
    fn from(char: char) -> Self {
        if char.is_numeric() {
            TokenType::Numeric
        } else if char == '*' {
            TokenType::Gear
        } else if char == '.' {
            TokenType::Dot
        } else {
            TokenType::Symbol
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Token {
    token_type: TokenType,
    value: String
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.token_type, self.value)
    }
}

impl Token {
    fn new(value: String) -> Self {
        if let Some(char) = value.chars().next() {
            Token {token_type: TokenType::from(char), value: value}
        } else {
            //empty string
            panic!("Can't construct a Token out of an empty String! {}", value);
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Point {
    y: usize,
    x: usize
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "y:{} x:{}", self.y, self.x)
    }
}

impl Point {
    fn new(y: usize, x: usize) -> Self {
        Point {y: y, x: x}
    }

    fn surrounding_range(&self, length: usize) -> (Point, Point) {
        (
            (Point::new(
                cmp::max(self.y, 1) - 1, 
                cmp::max(self.x, 1) - 1)), 
            (Point::new(self.y + 1, self.x + length)) //0 based!
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_grid() {
        Grid::new();
    }

    #[test]
    fn test_fill_map_from_text() -> Result<(), io::Error> {
        let file = File::open("res/day3.test.txt")?;
        let reader = BufReader::new(file);
        let mut grid = Grid::new();
        fill_map_from_text(Box::new(reader.lines()), &mut grid)?;
    
        let mut expected_map: BTreeMap<Point, Token> = BTreeMap::new();
        //inserting the following map:
        // 467..114..
        // ...*......
        // ..35..633.
        // ...abc#...
        expected_map.insert(Point::new(0, 0), Token::new("467".to_string()));
        expected_map.insert(Point::new(0, 5), Token::new("114".to_string()));
        expected_map.insert(Point::new(1, 3), Token::new("*".to_string()));
        expected_map.insert(Point::new(2, 2), Token::new("35".to_string()));
        expected_map.insert(Point::new(2, 6), Token::new("633".to_string()));
        
        //This follows the design decision to combine adjacent symbols:
        // expected_map.insert(Point::new(3, 6), Token::new("#".to_string()));
        expected_map.insert(Point::new(3, 3), Token::new("abc#".to_string()));
        // expected_map.insert(Point::new(3, 4), Token::new("b".to_string()));
        // expected_map.insert(Point::new(3, 5), Token::new("c".to_string()));

        assert_eq!(expected_map, grid.tokens);
        Ok(())
    }

    #[test]
    fn test_sum_gear_ratios() -> Result<(), io::Error>  {
        //inserting the following map:
        let input = "467..114..\n\
                    ...*......\n\
                    ..35..633.\n\
                    ......#...\n\
                    617*......\n\
                    .....+.58.\n\
                    ..592.....\n\
                    ......755.\n\
                    ...$.*....\n\
                    .664.598..";
        let lines = Box::new(input.split("\n")
            .map(|str| Ok(str.to_string())));
        let mut grid = Grid::new();
        fill_map_from_text(lines, &mut grid)?;
        //expected sum:
        //
        let sum = sum_gear_ratios(grid);
        assert_eq!(467 * 35 + 755 * 598, sum);
        Ok(())
    }

    #[test]
    fn test_map_range() {
        let mut map: BTreeMap<Point, Token> = BTreeMap::new();
        //inserting the following map:
        // 467..114..
        // ...*......
        // ..35..633.
        // ...abc#...
        let interesting_point = Point::new(2, 2);

        map.insert(Point::new(0, 0), Token::new("467".to_string()));
        map.insert(Point::new(0, 5), Token::new("114".to_string()));
        map.insert(Point::new(1, 3), Token::new("*".to_string()));
        map.insert(interesting_point, Token::new("35".to_string()));
        map.insert(Point::new(2, 6), Token::new("633".to_string()));
        // map.insert(Point::new(3, 6), Token::new("#".to_string()));
        map.insert(Point::new(3, 3), Token::new("abc#".to_string()));
        // map.insert(Point::new(3, 4), Token::new("b".to_string()));
        // map.insert(Point::new(3, 5), Token::new("c".to_string()));

        //now we want all elements surrounding "35"
        let surrounding_range: (Point, Point) = interesting_point.surrounding_range(2);
        assert_eq!((Point::new(1, 1)), surrounding_range.0);
        assert_eq!((Point::new(3, 4)), surrounding_range.1);

        let map_range: BTreeMap<Point, String> = map.range_mut((Included(surrounding_range.0), Included(surrounding_range.1)))
            .map(|(ptr, token)| (*ptr, format!("{} {}", token.value, token.token_type)))
            //can't use token directly, as I would have to copy it, as it's shared in the map and can't be derefed
            //I don't like that I have to implement the Copy Trait for Token here,
            //as I think the *token should be able to be consumered here to be put into the new map
            //doesn't work, as the range iterator doesn't allow me to remove elements from the underlying map 
            .fold(BTreeMap::new(), |mut map, (k, v)| {
                map.insert(k, v);
                return map;
            });

        assert_eq!("35 Numeric".to_string(), *map_range.get(&Point::new(2, 2)).unwrap());
        assert_eq!("* Gear".to_string(), *map_range.get(&Point::new(1, 3)).unwrap());
        assert_eq!("abc# Symbol".to_string(), *map_range.get(&Point::new(3, 3)).unwrap());
        // assert_eq!("b Symbol".to_string(), *map_range.get(&Point::new(3, 4)).unwrap());
    }

    #[test]
    fn test_map_range_sophisticated() -> Result<(), io::Error> {
        //inserting the following map:
        // x.x.x.x
        // .x.x.x.
        // x.123.x
        // .x.x.x.
        // x.x.x.x
        let file = File::open("res/day3.test2.txt")?;
        let reader = BufReader::new(file);
        let mut grid = Grid::new();
        fill_map_from_text(Box::new(reader.lines()), &mut grid)?;

        let interesting_point = Point::new(2, 2);
        println!("{} {}", interesting_point, "123");
        //get the range that draws the box around the token
        let surrounding_range = interesting_point.surrounding_range(3);
        assert_eq!((Point::new(1, 1)), surrounding_range.0);
        assert_eq!((Point::new(3, 5)), surrounding_range.1);
        println!("    bounds:");
        println!("      {:?}", surrounding_range.0);        
        println!("      {:?}", surrounding_range.1);
        
        //and iterate all items inside that range
        let surrounding_symbols: Vec<(&Point, &Token)> = get_in_range(&grid.tokens, &surrounding_range, TokenType::Symbol);
        // let surrounding_symbols: Vec<(&Point, &Token)> = map.range(surrounding_range)
        //     .filter(|(_, token)| token.token_type == TokenType::Symbol)
        //     .collect();
    
        println!("    surroundings:");
        for (p, t) in surrounding_symbols.clone() {
            println!("  {} {}", p, t);
        }

        // let map_range: BTreeMap<Point, String> = map.range_mut((Included(surrounding_range.0), Included(surrounding_range.1)))
        //     .map(|(ptr, token)| (*ptr, format!("{} {}", token.value, token.token_type)))
        //     //can't use token directly, as I would have to copy it, as it's shared in the map and can't be derefed
        //     //I don't like that I have to implement the Copy Trait for Token here,
        //     //as I think the *token should be able to be consumered here to be put into the new map
        //     //doesn't work, as the range iterator doesn't allow me to remove elements from the underlying map 
        //     .fold(BTreeMap::new(), |mut map, (k, v)| {
        //         map.insert(k, v);
        //         return map;
        //     });

        // assert_eq!("35 Numeric".to_string(), *map_range.get(&Point::new(2, 2)).unwrap());
        // assert_eq!("* Symbol".to_string(), *map_range.get(&Point::new(1, 3)).unwrap());
        // assert_eq!("abc# Symbol".to_string(), *map_range.get(&Point::new(3, 3)).unwrap());
        // assert_eq!("b Symbol".to_string(), *map_range.get(&Point::new(3, 4)).unwrap());
        Ok(())
    }
}