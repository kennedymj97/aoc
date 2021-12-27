pub fn p4a(input: &str) -> i32 {
    let (nums, mut boards) = parse_input(input);
    let (last_num, winning_board) = find_first_winner(nums, &mut boards).unwrap();
    score(last_num, &boards[winning_board as usize])
}

pub fn p4b(input: &str) -> i32 {
    let (nums, mut boards) = parse_input(input);
    let (last_num, winning_board) = find_last_winner(nums, &mut boards).unwrap();
    score(last_num, &boards[winning_board as usize])
}

fn score(last_num: i32, board: &Vec<Vec<i32>>) -> i32 {
    board
        .iter()
        .map(|row| row.iter().filter(|val| **val != -1).sum::<i32>())
        .sum::<i32>()
        * last_num
}

fn find_first_winner<'a>(
    nums: Vec<i32>,
    boards: &mut Vec<Vec<Vec<i32>>>,
) -> Result<(i32, i32), &'a str> {
    for num in nums {
        for i in 0..boards.len() {
            // cross number of boards
            boards[i] = boards[i]
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|&tile| if tile == num { -1 } else { tile })
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>();

            // check if a board has won
            for j in 0..boards[i].len() {
                let row_sum: i32 = boards[i][j].iter().sum();
                let col_sum: i32 = boards[i].iter().map(|row| row[j]).sum();
                if row_sum == -1 * boards[i].len() as i32 || col_sum == -1 * boards[i].len() as i32
                {
                    return Ok((num, i as i32));
                }
            }
        }
    }
    Err("no winner found")
}

fn find_last_winner<'a>(
    nums: Vec<i32>,
    boards: &mut Vec<Vec<Vec<i32>>>,
) -> Result<(i32, i32), &'a str> {
    let mut winning_boards = vec![0; boards.len()];
    for num in nums {
        for i in 0..boards.len() {
            boards[i] = boards[i]
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|&tile| if tile == num { -1 } else { tile })
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>();

            for j in 0..boards[i].len() {
                let row_sum: i32 = boards[i][j].iter().sum();
                let col_sum: i32 = boards[i].iter().map(|row| row[j]).sum();
                if row_sum == -1 * boards[i].len() as i32 || col_sum == -1 * boards[i].len() as i32
                {
                    winning_boards[i] = 1;
                    if winning_boards.iter().filter(|ele| **ele == 0).count() == 0 {
                        return Ok((num, i as i32));
                    }
                }
            }
        }
    }
    Err("no winner found")
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
    input
        .split("\n\n")
        .enumerate()
        .fold((vec![], vec![]), |(nums, mut boards), (i, section)| {
            if i == 0 {
                return (
                    section.split(',').map(|ele| ele.parse().unwrap()).collect(),
                    boards,
                );
            }
            boards.push(parse_board(section));
            (nums, boards)
        })
}

fn parse_board(board: &str) -> Vec<Vec<i32>> {
    board.lines().fold(vec![], |mut board, row| {
        board.push(
            row.split(' ')
                .filter_map(|ele| ele.parse().ok())
                .collect::<Vec<i32>>(),
        );
        board
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p4() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        assert_eq!(p4a(input), 4512);
        assert_eq!(p4b(input), 1924);
    }
}
