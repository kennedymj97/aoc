pub fn p2a(commands: &str) -> i32 {
    commands
        .lines()
        .fold(Pos { x: 0, y: 0 }, |pos, line| {
            let command = parse_command(line);
            match command {
                Command {
                    instruction: "forward",
                    value: dist,
                } => return pos.move_x(dist),
                Command {
                    instruction: "up",
                    value: dist,
                } => return pos.move_y(-dist),
                Command {
                    instruction: "down",
                    value: dist,
                } => return pos.move_y(dist),
                _ => panic!("unexpected input"),
            }
        })
        .product()
}

pub fn p2b(commands: &str) -> i32 {
    commands
        .lines()
        .fold(
            Navigation {
                pos: Pos { x: 0, y: 0 },
                aim: 0,
            },
            |acc, line| {
                let command = parse_command(line);
                match command {
                    Command {
                        instruction: "up",
                        value: val,
                    } => {
                        return Navigation {
                            pos: acc.pos,
                            aim: acc.aim - val,
                        }
                    }
                    Command {
                        instruction: "down",
                        value: val,
                    } => {
                        return Navigation {
                            pos: acc.pos,
                            aim: acc.aim + val,
                        }
                    }
                    Command {
                        instruction: "forward",
                        value: val,
                    } => {
                        return Navigation {
                            pos: Pos {
                                x: acc.pos.x + val,
                                y: acc.pos.y + (acc.aim * val),
                            },
                            aim: acc.aim,
                        }
                    }
                    _ => panic!("unexpected input"),
                }
            },
        )
        .pos
        .product()
}

struct Command<'a> {
    instruction: &'a str,
    value: i32,
}

#[derive(Debug, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn move_x(&self, dist: i32) -> Pos {
        Pos {
            x: self.x + dist,
            y: self.y,
        }
    }

    fn move_y(&self, dist: i32) -> Pos {
        Pos {
            x: self.x,
            y: self.y + dist,
        }
    }

    fn product(&self) -> i32 {
        self.x * self.y
    }
}

struct Navigation {
    pos: Pos,
    aim: i32,
}

fn parse_command(l: &str) -> Command {
    let test: Vec<&str> = l.split(' ').collect();
    Command {
        instruction: test[0],
        value: test[1].parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p2() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(p2a(input), Pos { x: 15, y: 10 }.product());
        assert_eq!(
            p2b(input),
            Navigation {
                pos: Pos { x: 15, y: 60 },
                aim: 10
            }
            .pos
            .product()
        );
    }
}
