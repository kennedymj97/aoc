use std::collections::HashMap;

pub fn p8a(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split_once(" | ").unwrap().1)
        .fold(0, |acc, out_digits| {
            acc + out_digits.split(' ').fold(0, |acc, out_digit| {
                acc + match out_digit.len() {
                    2 | 4 | 3 | 7 => 1,
                    _ => 0,
                }
            })
        })
}

pub fn p8b(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split_once(" | ").unwrap())
        .fold(0, |acc, (signals, out_digits)| {
            let m = digit_map(signals);
            acc + out_digits
                .split(' ')
                .map(sort_chars)
                .fold(String::new(), |acc, digit| {
                    format!("{}{}", acc, m.get(&digit).unwrap())
                })
                .parse::<u32>()
                .unwrap()
        })
}

fn digit_map(signals: &str) -> HashMap<String, &str> {
    let signals: Vec<&str> = signals.split(' ').collect();
    let mut num_to_sig: [&str; 10] = [""; 10];
    num_to_sig[1] = signals.iter().find(|signal| signal.len() == 2).unwrap();
    num_to_sig[4] = signals.iter().find(|signal| signal.len() == 4).unwrap();
    num_to_sig[7] = signals.iter().find(|signal| signal.len() == 3).unwrap();
    num_to_sig[8] = signals.iter().find(|signal| signal.len() == 7).unwrap();
    num_to_sig[9] = signals
        .iter()
        .find(|&&signal| signal.len() == 6 && is_subset(num_to_sig[4], signal))
        .unwrap();
    num_to_sig[0] = signals
        .iter()
        .find(|&&signal| {
            signal.len() == 6 && is_subset(num_to_sig[7], signal) && signal != num_to_sig[9]
        })
        .unwrap();
    num_to_sig[6] = signals
        .iter()
        .find(|&&signal| signal.len() == 6 && signal != num_to_sig[9] && signal != num_to_sig[0])
        .unwrap();
    num_to_sig[3] = signals
        .iter()
        .find(|&&signal| signal.len() == 5 && is_subset(num_to_sig[7], signal))
        .unwrap();
    num_to_sig[5] = signals
        .iter()
        .find(|&&signal| {
            signal.len() == 5 && signal != num_to_sig[3] && is_subset(signal, num_to_sig[9])
        })
        .unwrap();
    num_to_sig[2] = signals
        .iter()
        .find(|&&signal| signal.len() == 5 && signal != num_to_sig[3] && signal != num_to_sig[5])
        .unwrap();
    HashMap::from([
        (sort_chars(num_to_sig[0]), "0"),
        (sort_chars(num_to_sig[1]), "1"),
        (sort_chars(num_to_sig[2]), "2"),
        (sort_chars(num_to_sig[3]), "3"),
        (sort_chars(num_to_sig[4]), "4"),
        (sort_chars(num_to_sig[5]), "5"),
        (sort_chars(num_to_sig[6]), "6"),
        (sort_chars(num_to_sig[7]), "7"),
        (sort_chars(num_to_sig[8]), "8"),
        (sort_chars(num_to_sig[9]), "9"),
    ])
}

fn is_subset(sub: &str, main: &str) -> bool {
    for c_sub in sub.chars() {
        let mut match_found = false;
        for c_main in main.chars() {
            if c_sub == c_main {
                match_found = true;
                break;
            }
        }
        if !match_found {
            return false;
        }
    }
    true
}

fn sort_chars(s: &str) -> String {
    let mut s: Vec<char> = s.chars().collect();
    s.sort();
    s.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p8() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(p8a(input), 26);
        assert_eq!(p8b(input), 61229);
    }

    #[test]
    fn p8_subset() {
        assert!(is_subset("ab", "acdefb"));
    }

    #[test]
    fn p8_sort_chars() {
        assert_eq!(sort_chars("aedfbc"), "abcdef");
    }
}
