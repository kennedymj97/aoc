use std::collections::HashMap;
use std::iter::repeat;

pub fn p5a(input: &str) -> u32 {
    input
        .lines()
        .map(|l| points_a(l))
        .flatten()
        .fold(HashMap::new(), |mut pmap, p| {
            let count = pmap.entry(p).or_insert(0);
            *count += 1;
            pmap
        })
        .values()
        .filter(|c| **c > 1)
        .count() as u32
}

pub fn p5b(input: &str) -> u32 {
    input
        .lines()
        .map(|l| points_b(l))
        .flatten()
        .fold(HashMap::new(), |mut pmap, p| {
            let count = pmap.entry(p).or_insert(0);
            *count += 1;
            pmap
        })
        .values()
        .filter(|c| **c > 1)
        .count() as u32
}

fn points_a(line: &str) -> Vec<(u32, u32)> {
    let (p1, p2) = line.split_once(" -> ").unwrap();
    let (x1, y1) = p1.split_once(',').unwrap();
    let (x2, y2) = p2.split_once(',').unwrap();
    let (x1, x2, y1, y2) = (
        x1.parse::<u32>().unwrap(),
        x2.parse::<u32>().unwrap(),
        y1.parse::<u32>().unwrap(),
        y2.parse::<u32>().unwrap(),
    );
    let (x1, x2, y1, y2) = (x1.min(x2), x1.max(x2), y1.min(y2), y1.max(y2));
    if x1 == x2 {
        return repeat(x1).zip(y1..y2 + 1).collect();
    }
    if y1 == y2 {
        return (x1..x2 + 1).zip(repeat(y1)).collect();
    }
    vec![]
}

fn points_b(line: &str) -> Vec<(u32, u32)> {
    let (p1, p2) = line.split_once(" -> ").unwrap();
    let (x1, y1) = p1.split_once(',').unwrap();
    let (x2, y2) = p2.split_once(',').unwrap();
    let (x1, x2, y1, y2) = (
        x1.parse::<u32>().unwrap(),
        x2.parse::<u32>().unwrap(),
        y1.parse::<u32>().unwrap(),
        y2.parse::<u32>().unwrap(),
    );
    if x1 == x2 {
        return repeat(x1).zip(y1.min(y2)..y1.max(y2) + 1).collect();
    }
    if y1 == y2 {
        return (x1.min(x2)..x1.max(x2) + 1).zip(repeat(y1)).collect();
    }
    if x1 > x2 && y1 > y2 || x1 < x2 && y1 < y2 {
        return (x1.min(x2)..x1.max(x2) + 1)
            .zip(y1.min(y2)..y1.max(y2) + 1)
            .collect();
    }
    (x1.min(x2)..x1.max(x2) + 1)
        .zip((y1.min(y2)..y1.max(y2) + 1).rev())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!(p5a(input), 5);
        assert_eq!(p5b(input), 12);
    }
}
