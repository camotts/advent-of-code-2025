use nom::{
    character::complete::{line_ending, digit1},
    multi::{separated_list1},
    IResult,
};
type Output = Vec<Vec<i64>>;

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, list) = digit1(input)?;
    Ok((input, list.chars().map(|c| {
        c.to_digit(10).unwrap() as i64
    }).collect()))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let (input, list) = separated_list1(line_ending, parse_line)(input)?;
    Ok((input, list))
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Output {
    let _input = "987654321111111
811111111111119
234234234234278
818181911112111";
    let (input, output) = parse_input(input).expect("could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day3, part1)]
pub fn part1(input: &Output) -> i64 {
    input.iter().enumerate().map(|(_, bank)| {
        let max = bank.iter().rev().skip(1).rev().max().unwrap();
        let maxId = bank.iter().position(|i| i == max).unwrap();
        let second = bank.iter().skip(maxId+1).max().unwrap();
        (max * 10) + second
    }).sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &Output) -> i64 {
    input.iter().enumerate().map(|(_, bank)| {
        let mut nums: Vec<i64> = Vec::new();
        let mut skip = 0;
        for i in (0..12).rev() {
            let max = first_max(bank.iter().skip(skip).rev().skip(i).rev().collect()).unwrap();
            nums.push(max.1);
            skip = skip+max.0+1;
        }
        let ret = nums.iter().rev().fold((0, 1), |acc, &x| {
            (acc.0 + x * acc.1, acc.1*10)
        });
        ret.0
    }).sum::<i64>()
}

pub fn first_max(iterable: Vec<&i64>) -> Option<(usize, i64)>
{
    let mut max:Option<(usize, i64)> = None;
    iterable.into_iter().enumerate().for_each(|(i, &v)| {
        if max.clone().map_or(true, |other| 
            {
                v > other.1
            }) {

            max = Some((i, v.clone()));
        }
    });
    max
}