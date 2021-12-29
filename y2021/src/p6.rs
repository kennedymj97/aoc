pub fn p6a(input: &str) -> u64 {
    let mut fish: Vec<u64> = input.split(',').map(|v| v.parse().unwrap()).collect();
    for _ in 0..80 {
        fish = fish.iter().fold(vec![], |mut fish, life| {
            if *life == 0 {
                fish.append(&mut vec![6, 8]);
                return fish;
            }
            fish.push(*life - 1);
            fish
        });
    }
    fish.len() as u64
}

pub fn p6b(input: &str) -> u64 {
    let mut fish: [u64; 9] = input
        .split(',')
        .map(|ele| ele.parse::<usize>().unwrap())
        .fold([0; 9], |mut fish, life| {
            fish[life] += 1;
            fish
        });

    for _ in 0..256 {
        fish = (0..9).fold([0; 9], |mut new_fish, life| {
            if life == 0 {
                new_fish[6] += fish[0];
                new_fish[8] += fish[0];
            } else {
                new_fish[life - 1] += fish[life];
            }
            new_fish
        });
    }
    fish.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p6() {
        let input = "3,4,3,1,2";

        assert_eq!(p6a(input), 5934);
        assert_eq!(p6b(input), 26984457539);
    }
}
