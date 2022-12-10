use std::collections::HashSet;
use std::{error, fmt};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct DirectionParseError;

impl fmt::Display for DirectionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for DirectionParseError {}

#[derive(Debug, PartialEq, Eq)]
#[derive(Copy, Clone)]
#[derive(Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub(crate) fn follow(&mut self, to_be_followed: &Point) {
        if self.y - to_be_followed.y > 0 {
            self.y -= 1;
        } else if self.y - to_be_followed.y < 0 {
            self.y += 1;
        }
        if self.x - to_be_followed.x > 0 {
            self.x -= 1;
        } else if self.x - to_be_followed.x < 0 {
            self.x += 1;
        }
    }
}

impl Point {
    pub fn touching(self: &Point, other: &Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    pub fn move_me(self: &mut Point, dir: &Direction) -> () {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
    }
}

#[derive(Debug)]
pub struct Movement {
    direction: Direction,
    steps: usize,
}

impl Movement {
    pub fn parse(direction_str: &str, steps_str: &str) -> Result<Movement, Box<dyn error::Error>>
    {
        let direction = match direction_str {
            "D" => Direction::Down,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(DirectionParseError.into()),
        };
        let steps = steps_str.parse::<usize>().unwrap();

        return Ok(Movement { direction, steps });
    }
}

pub fn simulate_the_rope_return_tail_positions_part1(movements: Vec<Movement>) -> usize
{
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut tail_touched_positions: HashSet<Point> = vec![tail].into_iter().collect();

    for movement in movements.into_iter() {
        for _ in 0..movement.steps {
            head.move_me(&movement.direction);
            if !tail.touching(&head) {
                tail.follow(&head);
            }
            tail_touched_positions.insert(tail);
        }
    }
    return tail_touched_positions.len();
}

pub fn simulate_the_rope_return_tail_positions_part2(movements: Vec<Movement>) -> usize
{
    let mut rope = [Point { x: 0, y: 0 }; 10];
    let mut tail_touched_positions: HashSet<Point> = vec![rope[9]].into_iter().collect();

    for movement in movements.into_iter() {
        for _ in 0..movement.steps {
            rope[0].move_me(&movement.direction);
            for x in 0..9
            {
                if !rope[x + 1].touching(&rope[x]) {
                    let to_be_followed = rope[x].clone();
                    &rope[x + 1].follow(&to_be_followed);
                }
            }
            tail_touched_positions.insert(rope[9]);
        }
    }
    return tail_touched_positions.len();
}

pub(crate) fn parse_movements(lines: Vec<String>) -> Result<Vec<Movement>, Box<dyn error::Error>> {
    lines.iter().map(|line| line.split(" ").collect())
        .map(|vector: Vec<&str>| Movement::parse(vector[0], vector[1]))
        .collect::<Result<Vec<Movement>, Box<dyn error::Error>>>()
        .map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use crate::planks_09::Point;

    #[test]
    fn touching() {
        let a = Point { x: 32, y: 55 };
        let next_to_a = Point { x: 32, y: 56 };
        let diagonal_up_a = Point { x: 33, y: 56 };
        let one_two_far_right_from_a = Point { x: 34, y: 55 };
        assert_eq!(a.touching(&a), true);
        assert_eq!(a.touching(&next_to_a), true);
        assert_eq!(a.touching(&diagonal_up_a), true);
        assert_eq!(a.touching(&one_two_far_right_from_a), false);
        assert_eq!(a.touching(&one_two_far_right_from_a), false);
        let b = Point { x: -17, y: -1 };
        let next_to_b = Point { x: -16, y: -1 };
        let diagonal_down = Point { x: -17, y: -2 };
        let diagonal_up = Point { x: -17, y: 0 };
        assert_eq!(b.touching(&diagonal_down), true);
        assert_eq!(b.touching(&diagonal_up), true);
    }
}