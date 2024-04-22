use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::BTreeMap;
use std::ops::Bound::Included;
use std::ops::Bound;
use std::iter::Peekable;
use std::io::{self};
use std::iter::Enumerate;
use std::str::Chars;
use std::fmt;
use std::cmp;

fn main() -> std::io::Result<()> {
    //read each line,
    //parse anything that is not a .,
    //e.g. numbers, +, *, # etc.
    //safe the coordinates
    //calculate adjacency

    //imageine a grid 8x10
    //ranging from 0x0 to 7x9
    //each number e.g. 467 at 0x0 has adjacent fields,
    //to calculate them we draw a box around that field 

    //The schematic (the sheet 8x10) contains PartNumbers and Symbols
    //each SchematicElement has a (enum) Type PartNumbers or Symbols
    //a length and a coordinate Point (x=column, y=line)

    //add up the IDs of the games that would have been possible, you get 8.
        
    let file = File::open("res/day3_1.txt")?;
    let reader = BufReader::new(file);
    let mut map = BTreeMap::new();

    fill_map_from_text(reader, &mut map)?;

    let part_number_sum = sum_engine_parts(map);

    println!("Sum of numeric part numbers: {}", part_number_sum);
    Ok(())
}

fn sum_engine_parts(map: BTreeMap<Point, Token>) -> i32 {
    let mut part_number_sum = 0;
    //for each numeric token 
    let numeric_tokens = map.iter().filter(|(_, token)| token.token_type == TokenType::Numeric);
    for (point, token) in numeric_tokens {
        println!("{} {}", point, token);
        //get the range that draws the box around the token
        let surrounding_range: (Bound<Point>, Bound<Point>) = point.surrounding_range(token.value.len());
        println!("    bounds:");
        println!("      {:?}", surrounding_range.0);
        println!("      {:?}", surrounding_range.1);
        
        //and iterate all items inside that range
        let surrounding_symbols: Vec<(&Point, &Token)> = map.range(surrounding_range)
            .filter(|(_, token)| token.token_type == TokenType::Symbol)
            .collect();
    
        println!("    surroundings:");
        for (p, t) in surrounding_symbols.clone() {
            println!("  {} {}", p, t);
        }
        
        if !surrounding_symbols.is_empty() {
            let numeric_part_nr: i32 = token.value.parse().unwrap();
            part_number_sum += numeric_part_nr;
        }
    }
    part_number_sum
}

fn fill_map_from_text(reader: BufReader<File>, map: &mut BTreeMap<Point, Token>) -> Result<(), io::Error> {
    for (y, line) in reader.lines().enumerate() {
        let line: String = match line {
            Ok(line) => line,
            Err(e) => panic!("Error reading line {}", e)
        };
        let mut char_iter: Peekable<Enumerate<Chars>> = line.chars().enumerate().peekable();

        while let Some((x, char)) = char_iter.peek() {
            let x = *x;
            let char = *char;

            if char == '.' {
                //skip the dot
                char_iter.next(); 
                continue;
            }

            let token: Token;
            if char.is_numeric() {
                //e.g. 123 in ...123#..
                token = extract_numeric(&mut char_iter);
            } else {
                //e.g. # in ...123#..
                token = extract_symbol(&mut char_iter);
            }
            if let Some(old_value) = map.insert(Point::new(y, x), token) {
                panic!("Duplicate value in map, should not happen! {:?}", old_value);
            }
        }
    }
    Ok(())
}

// fn extract_numeric(iterator: Box<dyn Iterator<Item = (usize, char)>>) -> Token {
fn extract_numeric(iterator: &mut Peekable<Enumerate<Chars>>) -> Token {
    let result = extract_matching(iterator, |char| char.is_numeric());

    Token {
        token_type: TokenType::Numeric,
        value: result
    }
}

// fn extract_symbol(iterator: Box<dyn Iterator<Item = (usize, char)>>) -> Token {
fn extract_symbol(iterator: &mut Peekable<Enumerate<Chars>>) -> Token {
    let result = extract_matching(iterator, |char| (! char.is_numeric() && char != '.'));
    
    Token {
        token_type: TokenType::Symbol,
        value: result
    }
}

fn extract_matching<T: Fn(char) -> bool>(iterator: &mut Peekable<Enumerate<Chars>>, predicate: T) -> String {
    let mut result: String = String::new();

    while let Some((_, char)) = iterator.peek() {
        let char = *char;
        if predicate(char) {
            let (_, next_char) = iterator.next().expect("next should be present, if peek returned Some.");
            assert_eq!(char, next_char, "next and peek should be equal!");
            result.push(char);
        } else {
            break;
        }
    }

    result
}

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    Numeric,
    Symbol
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Numeric => write!(f, "Numeric"),
            TokenType::Symbol  => write!(f, "Symbol")
        }
    }
}

// struct 2dGrid {
//     x_map: HashMap<usize, Point>,
//     y_map: HashMap<usize, Point>,
//     tokens: HashMap<Point, Token>
// }
// impl 2dGrid {
//     fn find_in_range(from: Point, to: Point) {

//     }
// }

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
            if char.is_numeric() {
                Token {token_type: TokenType::Numeric, value: value}
            } else {
                Token {token_type: TokenType::Symbol, value: value}
            }
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
    fn test_extract_numeric() {
        let line = "123a";
        let mut char_iter: Peekable<Enumerate<Chars>> = line.chars().enumerate().peekable();

        let extracted = extract_numeric(&mut char_iter);
        assert_eq!(Token {token_type: TokenType::Numeric, value: "123".to_string()}, extracted);
        let (_, next_char) = char_iter.next().expect("iterator should have 'a' as next char.");
        assert_eq!('a', next_char);
    }

    #[test]
    fn test_extract_symbol() {
        let line = "#a123";
        let mut char_iter: Peekable<Enumerate<Chars>> = line.chars().enumerate().peekable();

        let extracted = extract_symbol(&mut char_iter);
        assert_eq!(Token {token_type: TokenType::Symbol, value: "#a".to_string()}, extracted);
        let (_, next_char) = char_iter.next().expect("iterator should have '1' as next char.");
        assert_eq!('1', next_char);
    }

    #[test]
    fn test_fill_map_from_text() -> Result<(), io::Error> {
        let file = File::open("res/day3.test.txt")?;
        let reader = BufReader::new(file);
        let mut actual_map = BTreeMap::new();
        fill_map_from_text(reader, &mut actual_map)?;
    
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

        assert_eq!(expected_map, actual_map);
        Ok(())
    }

    #[test]
    fn test_sum_engine_parts() {
        let mut map: BTreeMap<Point, Token> = BTreeMap::new();
        //inserting the following map:
        // 467..114..
        // ...*......
        // ..35..633.
        // ...ab.#...
        map.insert(Point::new(0, 0), Token::new("467".to_string()));
        map.insert(Point::new(0, 5), Token::new("114".to_string()));
        map.insert(Point::new(1, 3), Token::new("*".to_string()));
        map.insert(Point::new(2, 2), Token::new("35".to_string()));
        map.insert(Point::new(2, 6), Token::new("633".to_string()));
        map.insert(Point::new(3, 6), Token::new("#".to_string()));
        map.insert(Point::new(3, 3), Token::new("ab".to_string()));
        // map.insert(Point::new(3, 4), Token::new("b".to_string()));
        // map.insert(Point::new(3, 5), Token::new("c".to_string()));

        let sum = sum_engine_parts(map);
        assert_eq!(467 + 35 + 633, sum);
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
        assert_eq!(Included(Point::new(1, 1)), surrounding_range.0);
        assert_eq!(Included(Point::new(3, 4)), surrounding_range.1);

        let map_range: BTreeMap<Point, String> = map.range_mut(surrounding_range)
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
        assert_eq!("* Symbol".to_string(), *map_range.get(&Point::new(1, 3)).unwrap());
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
        let mut map = BTreeMap::new();
        fill_map_from_text(reader, &mut map)?;

        let interesting_point = Point::new(2, 2);
        println!("{} {}", interesting_point, "123");
        //get the range that draws the box around the token
        let surrounding_range = interesting_point.surrounding_range(3);
        assert_eq!(Included(Point::new(1, 1)), surrounding_range.0);
        assert_eq!(Included(Point::new(3, 5)), surrounding_range.1);
        println!("    bounds:");
        println!("      {:?}", surrounding_range.0);        
        println!("      {:?}", surrounding_range.1);
        
        //and iterate all items inside that range
        let surrounding_symbols: Vec<(&Point, &Token)> = get_symbols_in_range(&map, &surrounding_range);
        // let surrounding_symbols: Vec<(&Point, &Token)> = map.range(surrounding_range)
        //     .filter(|(_, token)| token.token_type == TokenType::Symbol)
        //     .collect();
    
        println!("    surroundings:");
        for (p, t) in surrounding_symbols.clone() {
            println!("  {} {}", p, t);
        }

        let map_range: BTreeMap<Point, String> = map.range_mut(surrounding_range)
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
        assert_eq!("* Symbol".to_string(), *map_range.get(&Point::new(1, 3)).unwrap());
        assert_eq!("abc# Symbol".to_string(), *map_range.get(&Point::new(3, 3)).unwrap());
        // assert_eq!("b Symbol".to_string(), *map_range.get(&Point::new(3, 4)).unwrap());
        Ok(())
    }
fn get_symbols_in_range<'a>(map: &'a BTreeMap<Point, Token>, range: &'a (Point, Point)) -> Vec<(&'a Point, &'a Token)> {
    let surrounding_symbols: Vec<(&Point, &Token)> = map.range((Included(range.0), Included(range.1)))
        .filter(|(_, token)| token.token_type == TokenType::Symbol)
        .filter(|(point, _)| (
            point.x > range.0.clone().x 
            && point.x < range.1.clone().x 
            && point.7 < range.1.clone().y
            && point.7 < range.1.clone()).y
        )
        .collect();
    surrounding_symbols
}
}