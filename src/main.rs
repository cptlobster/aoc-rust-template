/// Advent of Code puzzle solver
#[allow(dead_code, unused_imports, unused_variables)]
mod days;
mod common;

use common::read_input;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_parser)]
    day: i32,

    #[arg(short, long, value_parser)]
    part: i32,
}

/// Advent of Code puzzle solver
fn main() {
    let args = Args::parse();
    let result: i32 = get_solver(args.day, args.part);

    println!("Result of day {}, part {}: {}", args.day, args.part, result);
}

/// Find the correct solver for this day / part and return its result
fn get_solver(day: i32, part: i32) -> i32 {
    // TODO: rewrite this as a macro so I don't get publicly executed by every Rust dev that knows
    //   what they're doing
    match part {
        1 => match day {
            1  => solve("input/day01_1.txt", Box::new(days::day01::solve_1)),
            2  => solve("input/day02_1.txt", Box::new(days::day02::solve_1)),
            3  => solve("input/day03_1.txt", Box::new(days::day03::solve_1)),
            4  => solve("input/day04_1.txt", Box::new(days::day04::solve_1)),
            5  => solve("input/day05_1.txt", Box::new(days::day05::solve_1)),
            6  => solve("input/day06_1.txt", Box::new(days::day06::solve_1)),
            7  => solve("input/day07_1.txt", Box::new(days::day07::solve_1)),
            8  => solve("input/day08_1.txt", Box::new(days::day08::solve_1)),
            9  => solve("input/day09_1.txt", Box::new(days::day09::solve_1)),
            10 => solve("input/day10_1.txt", Box::new(days::day10::solve_1)),
            11 => solve("input/day11_1.txt", Box::new(days::day11::solve_1)),
            12 => solve("input/day12_1.txt", Box::new(days::day12::solve_1)),
            13 => solve("input/day13_1.txt", Box::new(days::day13::solve_1)),
            14 => solve("input/day14_1.txt", Box::new(days::day14::solve_1)),
            15 => solve("input/day15_1.txt", Box::new(days::day15::solve_1)),
            16 => solve("input/day16_1.txt", Box::new(days::day16::solve_1)),
            17 => solve("input/day17_1.txt", Box::new(days::day17::solve_1)),
            18 => solve("input/day18_1.txt", Box::new(days::day18::solve_1)),
            19 => solve("input/day19_1.txt", Box::new(days::day19::solve_1)),
            20 => solve("input/day20_1.txt", Box::new(days::day20::solve_1)),
            21 => solve("input/day21_1.txt", Box::new(days::day21::solve_1)),
            22 => solve("input/day22_1.txt", Box::new(days::day22::solve_1)),
            23 => solve("input/day23_1.txt", Box::new(days::day23::solve_1)),
            24 => solve("input/day24_1.txt", Box::new(days::day24::solve_1)),
            _ => panic!("day must be a valid number between 1 and 24")
        },
        2 => match day {
            1  => solve("input/day01_2.txt", Box::new(days::day01::solve_2)),
            2  => solve("input/day02_2.txt", Box::new(days::day02::solve_2)),
            3  => solve("input/day03_2.txt", Box::new(days::day03::solve_2)),
            4  => solve("input/day04_2.txt", Box::new(days::day04::solve_2)),
            5  => solve("input/day05_2.txt", Box::new(days::day05::solve_2)),
            6  => solve("input/day06_2.txt", Box::new(days::day06::solve_2)),
            7  => solve("input/day07_2.txt", Box::new(days::day07::solve_2)),
            8  => solve("input/day08_2.txt", Box::new(days::day08::solve_2)),
            9  => solve("input/day09_2.txt", Box::new(days::day09::solve_2)),
            10 => solve("input/day10_2.txt", Box::new(days::day10::solve_2)),
            11 => solve("input/day11_2.txt", Box::new(days::day11::solve_2)),
            12 => solve("input/day12_2.txt", Box::new(days::day12::solve_2)),
            13 => solve("input/day13_2.txt", Box::new(days::day13::solve_2)),
            14 => solve("input/day14_2.txt", Box::new(days::day14::solve_2)),
            15 => solve("input/day15_2.txt", Box::new(days::day15::solve_2)),
            16 => solve("input/day16_2.txt", Box::new(days::day16::solve_2)),
            17 => solve("input/day17_2.txt", Box::new(days::day17::solve_2)),
            18 => solve("input/day18_2.txt", Box::new(days::day18::solve_2)),
            19 => solve("input/day19_2.txt", Box::new(days::day19::solve_2)),
            20 => solve("input/day20_2.txt", Box::new(days::day20::solve_2)),
            21 => solve("input/day21_2.txt", Box::new(days::day21::solve_2)),
            22 => solve("input/day22_2.txt", Box::new(days::day22::solve_2)),
            23 => solve("input/day23_2.txt", Box::new(days::day23::solve_2)),
            24 => solve("input/day24_2.txt", Box::new(days::day24::solve_2)),
            _ => panic!("day must be a valid number between 1 and 24")
        },
        _ => panic!("part must equal either 1 or 2")
    }
}

/// Execute a solver on an input file and return its result
fn solve(input_file: &str, function: Box<dyn Fn(&str) -> i32>) -> i32 {
    let content: String = read_input(input_file);
    function(content.as_str())
}