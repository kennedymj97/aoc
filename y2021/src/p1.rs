pub fn p1a(depths: &str) -> u32 {
    count_depth_increases(depths.lines().map(|depth| depth.parse().unwrap()))
}

pub fn p1b(depths: &str) -> u32 {
    count_depth_increases_sliding_window(
        depths.lines().map(|depth| depth.parse().unwrap()).collect(),
    )
}

fn count_depth_increases<I: Iterator<Item = u32>>(depths: I) -> u32 {
    struct Acc {
        count: u32,
        prev: u32,
    }
    depths
        .fold(
            Acc {
                count: 0,
                prev: std::u32::MAX,
            },
            |acc, depth| {
                if depth > acc.prev {
                    return Acc {
                        count: acc.count + 1,
                        prev: depth,
                    };
                }
                return Acc {
                    count: acc.count,
                    prev: depth,
                };
            },
        )
        .count
}

fn count_depth_increases_sliding_window(depths: Vec<u32>) -> u32 {
    if depths.len() < 3 {
        return 0;
    }
    let mut increase_count = 0;
    for idx in 0..depths.len() - 3 {
        if depths[idx + 3] > depths[idx] {
            increase_count += 1;
        }
    }
    increase_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let all_increase = vec![1, 2, 3, 4, 5, 6, 7, 8]; // 7 increase
                                                         // sliding window all_increase
                                                         // 6, 9, 12, 15, 18, 21 // 5 increase
        let all_decrease = vec![8, 7, 6, 5, 4, 3, 2, 1]; // 0 increase
        let empty: Vec<u32> = vec![]; // 0 increase
        let mix = vec![1, 3, 2, 4, 5, 6, 1, 2, 0, 3, 1, 6, 8, 2]; // 8 increase
                                                                  // sliding window mix
                                                                  // 6, 9, 11, 15, 12, 9, 3, 5, 4, 10, 15, 16 // 7 increase
        let equal = vec![4, 4, 4, 4, 4, 4, 4]; // 0 increase

        assert_eq!(count_depth_increases(all_increase.clone().into_iter()), 7);
        assert_eq!(count_depth_increases(all_decrease.clone().into_iter()), 0);
        assert_eq!(count_depth_increases(empty.clone().into_iter()), 0);
        assert_eq!(count_depth_increases(mix.clone().into_iter()), 8);
        assert_eq!(count_depth_increases(equal.clone().into_iter()), 0);

        assert_eq!(count_depth_increases_sliding_window(all_increase), 5);
        assert_eq!(count_depth_increases_sliding_window(all_decrease), 0);
        assert_eq!(count_depth_increases_sliding_window(empty), 0);
        assert_eq!(count_depth_increases_sliding_window(mix), 7);
        assert_eq!(count_depth_increases_sliding_window(equal), 0);
    }
}
