mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

mod day_trait;

use day_trait::Day;
use std::fs;

fn write_result_to_file(day_num: u32, part: u32, result: &str) {
    let output_dir = format!("output/day{:02}", day_num);
    let output_file = format!("{}/part{:02}.txt", output_dir, part);
    fs::create_dir_all(&output_dir).expect("Unable to create directories");
    fs::write(output_file, result).expect("Unable to write file");
}

fn print_day(day_num: u32, part: u32) {
    let day: Box<dyn Day> = match day_num {
        1 => Box::new(day01::Day01),
        2 => Box::new(day02::Day02),
        3 => Box::new(day03::Day03),
        4 => Box::new(day04::Day04),
        5 => Box::new(day05::Day05),
        6 => Box::new(day06::Day06),
        7 => Box::new(day07::Day07),
        _ => panic!("Day not implemented"),
    };

    let result = match part {
        1 => day.part1(),
        2 => day.part2(),
        _ => panic!("Part not implemented"),
    };

    // Write the result to the output file
    write_result_to_file(day_num, part, &result);

    println!("Day {:02} Part {:02}: {}", day_num, part, result);
}

fn main() {
    print_day(7, 1);
    print_day(7, 2);
}
