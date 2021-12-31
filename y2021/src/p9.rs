pub fn p9a(heights: &str) -> u32 {
    let heights: Vec<Vec<u32>> = heights
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut mins: Vec<u32> = vec![];
    for i in 0..heights.len() {
        for j in 0..heights[0].len() {
            let adjacent_heights = [
                get_height(&heights, i as i32 - 1, j as i32),
                get_height(&heights, i as i32 + 1, j as i32),
                get_height(&heights, i as i32, j as i32 - 1),
                get_height(&heights, i as i32, j as i32 + 1),
            ];
            if adjacent_heights
                .iter()
                .all(|opt_h| opt_h.map_or(true, |&h| h > heights[i][j]))
            {
                mins.push(heights[i][j]);
            }
        }
    }
    mins.iter().fold(0, |acc, h| acc + 1 + h)
}

fn get_height<'a>(heights: &'a Vec<Vec<u32>>, i: i32, j: i32) -> Option<&'a u32> {
    if i < 0 || j < 0 {
        return None;
    }
    heights
        .get(i as usize)
        .map_or(None, |row| row.get(j as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p9() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(p9a(input), 15);
    }
}
