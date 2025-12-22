use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to open input.")
}

fn get_map(input: &str) -> Vec<Vec<char>> {
    let mut out: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        out.push(line.chars().collect());
    }

    out
}

fn get_boundaries(input: &str) -> (i64, i64) {
    let mut boundaries: (i64, i64) = (0, 0);

    for line in input.lines() {
        boundaries.1 += 1;
        let len = line.len() as i64;
        if len > boundaries.0 {
            boundaries.0 = len;
        }
    }

    boundaries
}

fn get_antennae(input: &str) -> HashMap<char, Vec<(i64, i64)>> {
    let mut antennae: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                let pos_vec = antennae.entry(c).or_insert(Vec::new());
                pos_vec.push((x as i64, y as i64));
            }
        }
    }

    antennae
}

fn get_offset(p1: (i64, i64), p2: (i64, i64)) -> (i64, i64) {
    (p2.0 - p1.0, p2.1 - p1.1)
}

fn get_antinodes(antennae: &[(i64, i64)], antinodes: &mut Vec<(i64, i64)>, bounds: (i64, i64)) {
    for i in 0..antennae.len() {
        for j in (i + 1)..antennae.len() {
            let p1 = antennae[i];
            let p2 = antennae[j];
            let offset = get_offset(p1, p2);

            let mut node = p1;
            while is_in_bounds(node, bounds) {
                antinodes.push(node);
                node = (node.0 - offset.0, node.1 - offset.1);
            }
            node = p2;
            while is_in_bounds(node, bounds) {
                antinodes.push(node);
                node = (node.0 + offset.0, node.1 + offset.1);
            }
            // antinodes.push((p1.0 - offset.0, p1.1 - offset.1));
            // antinodes.push((p2.0 + offset.0, p2.1 + offset.1));
        }
    }
}

fn is_in_bounds(pos: (i64, i64), bounds: (i64, i64)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < bounds.0 && pos.1 < bounds.1
}

fn main() {
    // let mut input = get_input("test.txt");
    let input = get_input("input.txt");

    let bounds = get_boundaries(&input);
    let antennae = get_antennae(&input);
    let mut antinodes: Vec<(i64, i64)> = Vec::new();

    for (key, val) in antennae {
        println!("--- [{key}] ---");
        get_antinodes(&val, &mut antinodes, bounds);
        for pos in val {
            println!("({},{})", pos.0, pos.1);
        }
    }
    println!();

    let antinodes: HashSet<_> = antinodes.into_iter().collect();
    let antinodes: Vec<(i64, i64)> = antinodes.into_iter().collect();
    let antinodes: Vec<&(i64, i64)> = antinodes
        .iter()
        .filter(|&x| is_in_bounds(*x, bounds))
        .collect();

    // for node in &antinodes {
    //     println!("({},{})", node.0, node.1);
    // }

    let mut map = get_map(&input);

    for node in &antinodes {
        map[node.1 as usize][node.0 as usize] = '#';
    }
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }

    println!("total: {}", antinodes.len());
}
