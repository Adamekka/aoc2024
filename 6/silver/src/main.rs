use std::collections::HashSet;

const MAP_SIZE: usize = 130;
type Map = [Option<Point>; MAP_SIZE * MAP_SIZE];

#[derive(Clone, Copy)]
enum Point {
    Guard(Dir),
    Obstacle,
}

trait Guard {
    fn is_guard(&self) -> bool;
    fn guard_dir(self) -> Dir;
    fn turn_right(&mut self);
}

impl Guard for Option<Point> {
    fn is_guard(&self) -> bool {
        matches!(self, Some(Point::Guard(_)))
    }

    fn guard_dir(self) -> Dir {
        match self {
            Some(p) => match p {
                Point::Guard(dir) => dir,
                Point::Obstacle => unreachable!(),
            },
            None => unreachable!(),
        }
    }

    fn turn_right(&mut self) {
        use Dir::*;
        if let Some(Point::Guard(ref mut dir)) = self {
            *dir = match *dir {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            };
        }
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").unwrap();

    println!("{}", solve(&text));
}

fn solve(input: &str) -> u16 {
    let mut map: Map = [None; MAP_SIZE * MAP_SIZE];

    for (line_idx, line) in input.lines().enumerate() {
        for (row_idx, c) in line.chars().enumerate() {
            map[line_idx * MAP_SIZE + row_idx] = match c {
                '#' => Some(Point::Obstacle),
                '^' => Some(Point::Guard(Dir::Up)),
                '.' => None,
                _ => unreachable!(),
            };
        }
    }

    let mut visited = HashSet::<u16>::new();

    while let Ok(v) = guard_move(&mut map) {
        if let Some(v) = v {
            visited.insert(v);
        }
    }

    (visited.len() + 1) as u16
}

fn guard_move(map: &mut Map) -> Result<Option<u16>, ()> {
    let pos = map.iter().position(|&p| p.is_guard()).unwrap();
    let line = pos / MAP_SIZE;
    let row = pos % MAP_SIZE;

    let guard = map[pos];

    assert!(guard.is_guard());

    let guard_dir = guard.guard_dir();

    let out_of_bounds = match guard_dir {
        Dir::Up => (line as isize - 1) < 0,
        Dir::Right => (row + 1) >= MAP_SIZE,
        Dir::Down => (line + 1) >= MAP_SIZE,
        Dir::Left => (row as isize - 1) < 0,
    };

    if out_of_bounds {
        return Err(());
    }

    let next_point_pos = match guard_dir {
        Dir::Up => (line - 1) * MAP_SIZE + row,
        Dir::Right => line * MAP_SIZE + (row + 1),
        Dir::Down => (line + 1) * MAP_SIZE + row,
        Dir::Left => line * MAP_SIZE + (row - 1),
    };

    let next_point = map[next_point_pos];

    match next_point {
        Some(p) => match p {
            Point::Obstacle => {
                map[pos].turn_right();
                return Ok(None);
            }
            Point::Guard(_) => unreachable!(),
        },
        None => {
            map[pos] = None;
            map[next_point_pos] = Some(Point::Guard(guard_dir));
        }
    }

    Ok(Some(next_point_pos as u16))
}
