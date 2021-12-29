pub fn p7a(input: &str) -> i32 {
    let input: Vec<i32> = input.split(',').map(|ele| ele.parse().unwrap()).collect();
    (*input.iter().min().unwrap()..*input.iter().max().unwrap() + 1).fold(
        std::i32::MAX,
        |min, pos| {
            input
                .iter()
                .fold(0, |acc, p| acc + (p - pos).abs())
                .min(min)
        },
    )
}

pub fn p7b(input: &str) -> i32 {
    let input: Vec<i32> = input.split(',').map(|ele| ele.parse().unwrap()).collect();
    (*input.iter().min().unwrap()..*input.iter().max().unwrap() + 1).fold(
        std::i32::MAX,
        |min, pos| {
            input
                .iter()
                .fold(0, |acc, p| {
                    let diff = (p - pos).abs();
                    acc + ((diff * (diff + 1)) / 2)
                })
                .min(min)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p7() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(p7a(input), 37);
        assert_eq!(p7b(input), 168);
    }
}
