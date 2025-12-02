use nom::{
    branch::alt,
    bytes::complete::tag,
    sequence::tuple,
    character::complete::{line_ending, anychar, digit1},
    multi::{many1, separated_list1},
    combinator::map_res,
    IResult, Parser,
};
use std::str::FromStr;

type Output = Vec<(i64, i64)>;

fn parse_line(input: &str) -> IResult<&str, (i64,i64)> {
    let(input, dir) = tuple((
        map_res(digit1, |s: &str| s.parse()),
        tag("-"),
        map_res(digit1, |s: &str| s.parse())))(input)?;
    Ok((input, (dir.0, dir.2)))
}


fn parse_input(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    let (input, list) = separated_list1(tag(","), parse_line)(input)?;
    Ok((input, list))
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Output {
    let _input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let (input, output) = parse_input(input).expect("could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day2, part1)]
pub fn part1(input: &Output) -> i64 {
    let mut res = 0;
    input.iter().enumerate().for_each(|(_, v)| {
        for i in v.0..v.1+1{
            let asstr = i.to_string();
            if asstr.len()%2 != 0 {
                continue
            }
            let (first, second) = asstr.split_at(asstr.len()/2);
            if first == second {
                res += i
            }
        }
    });
    res
}

#[aoc(day2, part2)]
pub fn part2(input: &Output) -> i64 {
    let mut res = 0;
    input.iter().enumerate().for_each(|(_, v)| {
        for i in v.0..v.1+1{
            let asstr = i.to_string();
            let mut acc = "".to_string();
            asstr.chars().enumerate().any(|(_, c)| {
                acc.push(c);
                if acc != asstr {
                    let new = asstr.replace(&acc, "");
                    if new == "" {
                        res += i;
                        return true
                    }
                }
                false
            });
        }
    });
    res
}