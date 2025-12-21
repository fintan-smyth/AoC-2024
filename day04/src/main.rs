use std::fs;

fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to open input.")
}

fn get_wordsearch(input: String) -> Vec<Vec<char>> {
    let mut out: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        out.push(line.chars().collect());
    }

    out
}

fn check_in_bounds(wordsearch: &[Vec<char>], pos: (i64, i64), offset: (i64, i64)) -> bool {
    let max_x = wordsearch[0].len() as i64;
    let max_y = wordsearch.len() as i64;
    let (x, y) = (pos.0 + offset.0, pos.1 + offset.1);

    if x >= max_x || x < 0 {
        return false;
    }
    if y >= max_y || y < 0 {
        return false;
    }

    true
}

fn check_letter(c: char, m_count: &mut i64, s_count: &mut i64) {
    match c {
        'M' => *m_count += 1,
        'S' => *s_count += 1,
        _ => (),
    }
}

fn check_mas(wordsearch: &[Vec<char>], pos: (i64, i64), offsets: &[(i64, i64)]) -> bool {
    let mut m_count = 0;
    let mut s_count = 0;
    let (x, y) = pos;

    for offset in offsets {
        if check_in_bounds(wordsearch, pos, *offset) {
            check_letter(
                wordsearch[(y + offset.1) as usize][(x + offset.0) as usize],
                &mut m_count,
                &mut s_count,
            );
        }
    }

    if m_count == 1 && s_count == 1 {
        return true;
    }
    false
}

fn find_mas_x(wordsearch: &[Vec<char>], pos: (i64, i64)) -> bool {
    let offsets: [(i64, i64); 4] = [(-1, -1), (1, 1), (1, -1), (-1, 1)];

    if !check_mas(wordsearch, pos, &offsets[0..2]) {
        return false;
    }
    if !check_mas(wordsearch, pos, &offsets[2..4]) {
        return false;
    }

    true
}

fn find_xmas(wordsearch: &[Vec<char>], pos: (i64, i64)) -> i64 {
    let mut matches = [true; 8];
    let xmas = "XMAS";
    let (x, y) = pos;

    for i in 1..4 {
        if !check_in_bounds(wordsearch, pos, (0, i as i64))
            || wordsearch[y as usize + i][x as usize] != xmas.chars().nth(i).unwrap()
        {
            matches[0] = false;
        }
        if !check_in_bounds(wordsearch, pos, (0, -(i as i64)))
            || wordsearch[y as usize - i][x as usize] != xmas.chars().nth(i).unwrap()
        {
            matches[1] = false;
        }
        if !check_in_bounds(wordsearch, pos, (i as i64, 0))
            || wordsearch[y as usize][x as usize + i] != xmas.chars().nth(i).unwrap()
        {
            matches[2] = false;
        }
        if !check_in_bounds(wordsearch, pos, (-(i as i64), 0))
            || wordsearch[y as usize][x as usize - i] != xmas.chars().nth(i).unwrap()
        {
            matches[3] = false;
        }
        if !check_in_bounds(wordsearch, pos, (i as i64, i as i64))
            || wordsearch[y as usize + i][x as usize + i] != xmas.chars().nth(i).unwrap()
        {
            matches[4] = false;
        }
        if !check_in_bounds(wordsearch, pos, (-(i as i64), -(i as i64)))
            || wordsearch[y as usize - i][x as usize - i] != xmas.chars().nth(i).unwrap()
        {
            matches[5] = false;
        }
        if !check_in_bounds(wordsearch, pos, (i as i64, -(i as i64)))
            || wordsearch[y as usize - i][x as usize + i] != xmas.chars().nth(i).unwrap()
        {
            matches[6] = false;
        }
        if !check_in_bounds(wordsearch, pos, (-(i as i64), i as i64))
            || wordsearch[y as usize + i][x as usize - i] != xmas.chars().nth(i).unwrap()
        {
            matches[7] = false;
        }
    }

    matches.iter().filter(|b| **b).count() as i64
}

fn print_square(wordsearch: &[Vec<char>], pos: (usize, usize)) {
    let (x, y) = pos;

    println!();
    for (i, row) in wordsearch[(y - 1)..=(y + 1)].iter().enumerate() {
        for (j, c) in row[(x - 1)..=(x + 1)].iter().enumerate() {
            if (j % 2 == 0 && i % 2 == 0) || (j == 1 && i == 1) {
                print!("{c}");
            } else {
                print!(" ");
            };
        }
        println!();
    }
}

fn main() {
    // let input = get_input("test.txt");
    let input = get_input("input.txt");

    println!("{input}");

    let wordsearch = get_wordsearch(input);
    let mut xmas = 0;
    let mut mas_x = 0;

    for y in 0..wordsearch.len() {
        for x in 0..wordsearch[y].len() {
            if wordsearch[y][x] == 'X' {
                let matches = find_xmas(&wordsearch, (x as i64, y as i64));
                // println!("({x},{y})  {matches}");
                xmas += matches;
            } else if wordsearch[y][x] == 'A' {
                let is_mas_x = find_mas_x(&wordsearch, (x as i64, y as i64));
                if is_mas_x {
                    mas_x += 1;
                    print_square(&wordsearch, (x, y));
                }
            }
        }
    }

    println!("xmas: {xmas}");
    println!("mas_x: {mas_x}");
}
