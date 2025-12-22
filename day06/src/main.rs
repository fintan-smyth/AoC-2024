use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    North,
    South,
    East,
    West,
}

fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to open input.")
}

fn get_map(input: String) -> Vec<Vec<char>> {
    let mut out: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        out.push(line.chars().collect());
    }

    out
}

fn get_start_pos(map: &[Vec<char>]) -> Option<(usize, usize)> {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '^' {
                return Some((x, y));
            };
        }
    }
    None
}

fn check_finished(map: &[Vec<char>], pos: &(usize, usize), dir: Dir) -> bool {
    let (x, y) = *pos;

    match dir {
        Dir::North => y == 0,
        Dir::South => y == (map.len() - 1),
        Dir::East => x == (map[0].len() - 1),
        Dir::West => x == 0,
    }
}

fn take_step(map: &[Vec<char>], pos: &mut (usize, usize), dir: &mut Dir) {
    let (x, y) = *pos;

    match dir {
        Dir::North => {
            if map[y - 1][x] == '#' {
                *dir = Dir::East;
            } else {
                pos.1 -= 1
            };
        }
        Dir::South => {
            if map[y + 1][x] == '#' {
                *dir = Dir::West;
            } else {
                pos.1 += 1
            };
        }
        Dir::East => {
            if map[y][x + 1] == '#' {
                *dir = Dir::South;
            } else {
                pos.0 += 1
            };
        }
        Dir::West => {
            if map[y][x - 1] == '#' {
                *dir = Dir::North;
            } else {
                pos.0 -= 1
            };
        }
    }
}

fn perform_walk(map: &mut [Vec<char>]) {
    let mut pos = get_start_pos(map).expect("No starting pos!");
    let mut dir = Dir::North;

    loop {
        map[pos.1][pos.0] = 'X';
        if check_finished(map, &pos, dir) {
            return;
        };
        take_step(map, &mut pos, &mut dir);
    }
}

fn pos_encountered(positions: &[(usize, usize, Dir)], pos: &(usize, usize, Dir)) -> bool {
    for position in positions {
        if position.0 == pos.0 && position.1 == pos.1 && position.2 == pos.2 {
            return true;
        }
    }

    false
}

fn does_walk_loop(map: Vec<Vec<char>>) -> bool {
    let mut positions: Vec<(usize, usize, Dir)> = Vec::new();
    let mut pos = get_start_pos(&map).expect("No starting pos!");
    let mut dir = Dir::North;

    loop {
        let cur_pos: (usize, usize, Dir) = (pos.0, pos.1, dir);
        if pos_encountered(&positions, &cur_pos) {
            return true;
        }
        positions.push(cur_pos);
        if check_finished(&map, &pos, dir) {
            return false;
        };
        take_step(&map, &mut pos, &mut dir);
    }
}

fn count_loop_positions(map: Vec<Vec<char>>) -> i64 {
    let start_pos = get_start_pos(&map).expect("No starting pos!");
    let mut count = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if x == start_pos.0 && y == start_pos.1 {
                continue;
            }
            let mut copy: Vec<Vec<char>> = map.clone();
            copy[y][x] = '#';
            if does_walk_loop(copy) {
                count += 1;
                println!("found loop! count: {count}");
            }
        }
    }

    count
}

fn count_positions(map: &[Vec<char>]) -> i64 {
    let mut count: i64 = 0;

    for row in map {
        for c in row {
            if *c == 'X' {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    // let input = get_input("test.txt");
    let input = get_input("input.txt");

    let mut map = get_map(input);

    // perform_walk(&mut map);
    let count = count_loop_positions(map);

    println!("count: {}", count);
}
