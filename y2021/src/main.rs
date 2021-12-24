use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use y2021::p1;

fn main() {
    println!(
        "{}",
        p1::count_depth_increases(load_input(Path::new("./inputs/p1.txt")))
    );
    println!(
        "{}",
        p1::count_depth_increases_sliding_window(load_input(Path::new("./inputs/p1.txt")))
    );
}

fn load_input(path: &Path) -> Vec<i32> {
    let file = File::open(path).expect("failed to open file");
    BufReader::new(file)
        .lines()
        .map(|l| {
            l.expect("unable to parse line")
                .parse()
                .expect("failed to get num from str")
        })
        .collect()
}
