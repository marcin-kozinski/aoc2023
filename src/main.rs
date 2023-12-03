mod day01;
mod day02;

fn main() {
    match "latest" {
        "01/1" => day01::part1(),
        "01/2" => day01::part2(),
        "02/1" => day02::part1(),
        "02/2" | _ => day02::part2(),
    }
}
