use std::{collections::HashSet, fs};

fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to open input.")
}

fn get_map(input: String) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<u32> = Vec::new();

        for c in line.chars() {
            row.push(c.to_digit(10).expect("Failed to convert digit"));
        }
        map.push(row);
    }

    map
}

fn walk_trail(
    map: &[Vec<u32>],
    pos: (usize, usize),
    height: u32,
    reachable: &mut HashSet<(usize, usize)>,
    n_paths: &mut i64,
) {
    if height == 9 {
        reachable.insert(pos);
        *n_paths += 1;
        return;
    }
    let (x, y) = pos;
    if y > 0 && map[y - 1][x] == height + 1 {
        walk_trail(map, (x, y - 1), height + 1, reachable, n_paths);
    }
    if y < map.len() - 1 && map[y + 1][x] == height + 1 {
        walk_trail(map, (x, y + 1), height + 1, reachable, n_paths);
    }
    if x > 0 && map[y][x - 1] == height + 1 {
        walk_trail(map, (x - 1, y), height + 1, reachable, n_paths);
    }
    if x < map[0].len() - 1 && map[y][x + 1] == height + 1 {
        walk_trail(map, (x + 1, y), height + 1, reachable, n_paths);
    }
}

fn score_trailhead(map: &[Vec<u32>], pos: (usize, usize)) -> (usize, i64) {
    let mut reachable: HashSet<(usize, usize)> = HashSet::new();
    let mut n_paths: i64 = 0;

    walk_trail(map, pos, 0, &mut reachable, &mut n_paths);
    (reachable.len(), n_paths)
}

fn main() {
    // let input = get_input("test.txt");
    let input = get_input("input.txt");

    let map = get_map(input);

    let mut total_score = 0;
    let mut total_rating = 0;

    for (y, v) in map.iter().enumerate() {
        for (x, num) in v.iter().enumerate() {
            if *num == 0 {
                let (score, rating) = score_trailhead(&map, (x, y));
                println!("({},{})\tscore: {score} rating: {rating}", x, y);
                total_score += score;
                total_rating += rating;
            }
        }
    }

    println!("total_score: {total_score}");
    println!("total_rating: {total_rating}");
}
