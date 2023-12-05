mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    match "latest" {
        "01/1" => day01::part1(),
        "01/2" => day01::part2(),
        "02/1" => day02::part1(),
        "02/2" => day02::part2(),
        "03/1" => day03::part1(),
        "03/2" => day03::part2(),
        "04/1" => day04::part1(),
        "04/2" | _ => day04::part2(),
    }
}
