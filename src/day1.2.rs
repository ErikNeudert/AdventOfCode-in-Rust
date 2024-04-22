use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("res/day1.2.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for maybe_line in reader.lines() {
        let line = maybe_line?;
        //parse first number
        let first_number = find_first_number(&line);
        let last_number = find_last_number(&line);
        // println!("{} {}", first_number, last_number);
        let combined_number = first_number * 10 + last_number;
        sum += combined_number;
    }
    println!("{}", sum);

    Ok(())
}

fn find_first_number(input: &str) -> i32 {
    return find_number(input, |iteration_range| iteration_range);
}
fn find_last_number(input: &str) -> i32 {
    return find_number(input, |iteration_range| Box::from(iteration_range.rev()));
}

const NUMBER_NAMES: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn find_number<F>(input: &str, iterator_modifier: F) -> i32 
    where F: Fn(Box<dyn DoubleEndedIterator<Item=usize>>) -> Box<dyn DoubleEndedIterator<Item=usize>> {
    let char_array = input.chars().collect::<Vec<char>>();

    for i in iterator_modifier(Box::from(0..char_array.len())) {
        let char = char_array[i];

        if char >= '0' && char <= '9' {
            return (char as i32) - ('0' as i32);
        } 

        'number_loop: for number_name in NUMBER_NAMES.iter().enumerate() {
            let (num_num, num_name) = number_name;
            
            let mut num_index = i;
            for num_char in num_name.chars() {
                if num_index >= char_array.len() {
                    continue 'number_loop;
                }
                if num_char != char_array[num_index] {
                    continue 'number_loop;
                }
                num_index += 1;
            }
            //enumeration starts at 0, therefor add 1
            return num_num as i32 + 1;
        }
    }

    panic!("No number found in input");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_number() {
        let input = "abcone2threexyz";
        let result = super::find_first_number(input);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_find_number2() {
        let input = "abconxe2threexyz";
        let result = super::find_first_number(input);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_find_number3() {
        let input = "abconxethreexyz";
        let result = super::find_first_number(input);
        assert_eq!(result, 3);
    }
    #[test]
    fn test_find_last_number1() {
        let input = "1234";
        let result = super::find_last_number(input);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_find_last_number2() {
        let input = "124three";
        let result = super::find_last_number(input);
        assert_eq!(result, 3);
    }
}