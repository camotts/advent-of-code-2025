use nom::{
    sequence::tuple,
    character::complete::{line_ending, anychar, digit1},
    multi::{separated_list1},
    IResult,
};
use std::str::FromStr;

type Output = Vec<Command>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Command {
    Left(i32),
    Right(i32),
}

fn parse_line2(input: &str) -> IResult<&str, Command> {
    let(input, dir) = tuple((anychar, digit1))(input)?;
    let num: i32 = FromStr::from_str(dir.1).unwrap();
    let cmd = match dir.0 {
        'L' => Command::Left(num),
        'R' => Command::Right(num),
        _ => panic!()
    };
    Ok((input, cmd))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, list) = separated_list1(line_ending, parse_line2)(input)?;
    Ok((input, list))
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Output {
    let _input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    let (input, output) = parse_input(input).expect("could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day1, part1)]
pub fn part1(input: &Output) -> u32 {
    let mut value: i32 = 50;
    let mut count = 0;
    input.iter().enumerate().for_each(|(_, c)| {
        match c {
            Command::Left(v) => {
                value = value - v % 100;
                if value < 0 {
                    value = 100 + value
                }
            },
            Command::Right(v) => {
                value = value + v % 100;
                if value > 99 {
                    value = value - 100
                }
            }
        }
        if value < 0 || value > 99 {
            println!("{:?}", value);
        }
        if value == 0 {
            count = count + 1
        }
    });
    count
}

#[aoc(day1, part2)]
pub fn part2(input: &Output) -> i32 {
    let mut value: i32 = 50;
    let mut count = 0;
    input.iter().enumerate().for_each(|(_, c)| {
        match c {
            Command::Left(v) => {
                let sv = value;
                count = count + (v / 100);
                value = value - v % 100;
                if value < 0 {
                    if sv != 0 {
                        count = count + 1;
                    }
                    value = 100 + value;
                    if value == 0 {
                        count = count -1;
                    }
                }
            },
            Command::Right(v) => {
                let sv = value;
                count = count + (v / 100);
                value = value + v % 100;
                if value > 99 {
                    if sv != 0 {
                        count = count + 1;
                    }
                    value = value - 100;
                    if value == 0 {
                        count = count -1;
                    }
                }
            }
        }
        if value == 0 {
            count = count + 1
        }
    });
    count
}

