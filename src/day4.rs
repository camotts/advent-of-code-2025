use nom::{
    IResult,
    character::complete::{line_ending, none_of},
    multi::{many1, separated_list1},
};

type Output = Vec<Vec<Space>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Space {
    Paper(u32),
    Empty,
}

fn parse_character(input: &str) -> IResult<&str, Space> {
    let(input, c) = none_of("\n")(input)?;
    let cmd = match c {
        '@' => Space::Paper(0),
        '.' => Space::Empty,
        _ => panic!("Found unexpected charater: {:?}", c)
    };
    Ok((input, cmd))
}

fn parse_line(input: &str) -> IResult<&str, Vec<Space>> {
    let(input, c) = many1(parse_character)(input)?;
    Ok((input, c))
}


fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Space>>> {
    let (input, list) = separated_list1(line_ending, parse_line)(input)?;
    Ok((input, list))
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Output {
    let _input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    let (input, output) = parse_input(input).expect("could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day4, part1)]
pub fn part1(input: &Output) -> u32 {
    let mut grid = input.clone();
    let ret = input.iter().enumerate().map(|(i, rows)| {
        rows.iter().enumerate().map(|(j, c)| {
            if matches!(*c, Space::Paper(_)) {
                grid[i][j] = Space::Paper(check_directions(&grid, i as i32, j as i32));
            }
            if matches!(*c, Space::Paper(_)) && check_directions(input, i as i32, j as i32) < 4 {
                println!("{:?}, {:?}: {:?}", i, j, check_directions(input, i as i32, j as i32));
                1
            } else {
                0
            }
        }).sum::<u32>()
    }).sum();
    print_grid(&grid);
    ret
}

#[aoc(day4, part2)]
pub fn part2(input: &Output) -> u32 {
    let mut grid = input.clone();
    let mut cont = true;
    let mut ret = 0;
    while cont {
        let innerIter = grid.clone();
        ret = ret + innerIter.iter().enumerate().map(|(i, rows)| {
            rows.iter().enumerate().map(|(j, c)| {
                let mut ret = 0;
                if matches!(*c, Space::Paper(_)) && check_directions(&grid, i as i32, j as i32) < 4 {
                    ret = 1
                }
                if matches!(*c, Space::Paper(_)) {
                    grid[i][j] = Space::Paper(check_directions(&grid, i as i32, j as i32));
                }
                ret
            }).sum::<u32>()
        }).sum::<u32>();
        (grid, cont) = remove_rolls(&grid);
    }
    ret
}

fn check_directions(grid: &Output, i: i32, j: i32) -> u32 {
    let mut ret:u32 = 0;
    for x in i-1..=i+1 {
        for y in j-1..=j+1 {
            if grid.get(x as usize).
            and_then(|row|row.get(y as usize).map(|cell| matches!(*cell, Space::Paper(_))))
            .unwrap_or(false) {
                ret += 1
            }
        }
    }
    ret.saturating_sub(1)
}

fn remove_rolls(grid: &Output) -> (Output, bool) {
    let mut out = grid.clone();
    let mut changed = false;
    out = out.iter().map(|row| {
        row.iter().map(|&cell| {
            match cell {
                Space::Empty => cell,
                Space::Paper(v) => {
                    if v < 4 {
                        changed = true;
                        Space::Empty
                    } else {
                        cell
                    }
                }
            }
        }).collect()
    }).collect();
    (out, changed)
}

fn print_grid(grid: &Output) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|cell| {
            match cell {
                Space::Empty => print!("."),
                Space::Paper(v) => {
                    if *v < 4 {
                        print!("x")
                    } else {
                        print!("@")
                    }
                }
            }
        });
        println!("")
    })
}