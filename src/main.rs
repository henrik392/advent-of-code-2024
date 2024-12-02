mod day_trait;
mod day01;
mod day02;

use day_trait::Day;

fn print_day(day_num: u32, part: u32) {
    let day = match day_num {
        1 => day01::Day01,
        _ => panic!("Day not implemented"),
    };

    let result = match part {
        1 => day.part1(),
        2 => day.part2(),
        _ => panic!("Part not implemented"),
    };

    println!("Day {:02} Part {:02}: {}", day_num, part, result);
}

fn main() {
    print_day(1, 1);
    print_day(1, 2);
}



