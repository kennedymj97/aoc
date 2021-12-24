pub fn p3a(report: &str) -> i32 {
    p3a_aux(report, 12)
}

fn p3a_aux(report: &str, width: usize) -> i32 {
    let (gamma, epsillon) = report
        .lines()
        .fold(vec![0; width], |acc, bits| {
            acc.iter()
                .enumerate()
                .map(|(i, sum)| match bits.chars().nth(i).unwrap() {
                    '0' => sum - 1,
                    '1' => sum + 1,
                    _ => unreachable!(),
                })
                .collect()
        })
        .iter()
        .fold((String::new(), String::new()), |(g, e), count| {
            if *count < 0 {
                (format!("{}{}", g, 0), format!("{}{}", e, 1))
            } else {
                (format!("{}{}", g, 1), format!("{}{}", e, 0))
            }
        });
    i32::from_str_radix(gamma.as_str(), 2).unwrap()
        * i32::from_str_radix(epsillon.as_str(), 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p3() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!(p3a_aux(input, 5), 198)
    }
}
