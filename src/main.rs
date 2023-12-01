mod day01;

fn main() {
    match "latest" {
        "01/1" => day01::part1(),
        "01/2" | _ => day01::part2(),
    }
}
