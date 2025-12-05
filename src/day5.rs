use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    IResult,
    character::complete::{anychar, digit1, line_ending},
    multi::separated_list1,
    sequence::{self, tuple},
};
use std::str::FromStr;
use std::collections::HashSet;

type Output = Data;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Data {
    db: Vec<Range>,
    ingredients: Vec<u128>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Range {
    min: u128,
    max: u128
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, (min, _, max)) = tuple((digit1, tag("-"), digit1))(input)?;
    Ok((input, Range{
        min: min.parse::<u128>().unwrap(),
        max: max.parse::<u128>().unwrap()
    }))
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<Range>> {
    let (input, ingredients) = separated_list1(line_ending, parse_range)(input)?;
    Ok((input, ingredients))
}

fn parse_ingredients(input: &str) -> IResult<&str, Vec<u128>> {
    let (input, ingredients) = separated_list1(line_ending, digit1)(input)?;
    Ok((input, ingredients.iter().map(|&s| {
        s.parse::<u128>().unwrap()
    }).collect()))
}

fn parse_input(input: &str) -> IResult<&str, Data> {
    //let (input, list) = separated_list1(line_ending, parse_line2)(input)?;
    let (input, (ingredientRanges, _, ingredients)) = tuple((parse_ranges, tag("\n\n"), parse_ingredients))(input)?;
    Ok((input, Data{
        db: ingredientRanges,
        ingredients: ingredients
    }))
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Output {
    let _input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    let (input, output) = parse_input(input).expect("could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day5, part1)]
pub fn part1(input: &Output) -> u128 {
    input.ingredients.iter().map(|&ingredient| {
        if input.db.iter().any(|range| {
            ingredient >= range.min && ingredient <= range.max
        }) {
            1
        } else {
            0
        }
    }).sum()
}

#[aoc(day5, part2)]
pub fn part2(input: &Output) -> u128 {
    let sorted: Vec<&Range> = input.db.iter().sorted_by(|&c, &v| {
        if c.min>v.min {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }).collect();
    let mut compoundRanges: Vec<Range> = vec![];
    compoundRanges.push(sorted.first().map(|&v| v.clone()).unwrap());
    for i in 1..sorted.len() {
        let mut idx = 0;
        if compoundRanges.iter().enumerate().any(|(ci, cr)| {
            if sorted[i].min >= cr.min && sorted[i].min <= cr.max {
                idx = ci;
                true
            } else{
                false
            }
        }) {
            let mut tmp = compoundRanges[idx].clone();
            tmp.max = sorted[i].max;
            compoundRanges[idx] = tmp;
        } else {
            println!("Making neww compound range: {:?}\n{:?}\n\n", sorted[i], compoundRanges);
            compoundRanges.push(sorted[i].clone());
        }
    };
    println!("{:?}, {:?}", compoundRanges, compoundRanges.len());
    compoundRanges.iter().map(|range| {
        range.max - range.min
    }).sum::<u128>()
}

pub fn part3(input: &Output) -> usize {
    let mut fresh: HashSet<u128> = HashSet::new();
    let mut test: Vec<Dat> = Vec::new();
    input.db.iter().for_each(|range| {
        test.push(Dat{sig:RangeEN::Min, num:range.min});
        test.push(Dat{sig:RangeEN::Max, num:range.max});
    });
    test.sort();
    let mut next: Vec<Dat> = Vec::new();
    next.push(*test.last().unwrap());
    for i in (0..test.len()-1).rev() {
        if matches!(test[i].sig, RangeEN::Min) {
           next.insert(0, test[i]); 
        }
    }
    println!("{:?}", test);
    fresh.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Dat {
    num: u128,
    sig: RangeEN
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum RangeEN {
    Min,
    Max
}