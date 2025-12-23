use std::fs;

fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to open input.")
}

fn build_disk(input: String) -> Vec<i64> {
    let mut disk: Vec<i64> = Vec::new();
    let mut chars = input.chars();
    let mut id = 0;

    while let Some(digit) = chars.next() {
        let mut num: i64 = digit.to_digit(10).expect("Failed to convert digit") as i64;
        for _ in 0..num {
            disk.push(id);
        }
        id += 1;
        if let Some(digit) = chars.next() {
            if digit == '\n' {
                break;
            }
            num = digit.to_digit(10).expect("Failed to convert digit") as i64;
        } else {
            break;
        }
        for _ in 0..num {
            disk.push(-1);
        }
    }

    disk
}

fn fragment(disk: &mut [i64]) {
    let mut last_block = disk
        .iter()
        .rposition(|&x| x != -1)
        .expect("No non-empty blocks!");
    let mut first_empty = disk
        .iter()
        .position(|&x| x == -1)
        .expect("No empty blocks!");

    while last_block > first_empty {
        disk.swap(last_block, first_empty);
        last_block = disk
            .iter()
            .rposition(|&x| x != -1)
            .expect("No non-empty blocks!");
        first_empty = disk
            .iter()
            .position(|&x| x == -1)
            .expect("No empty blocks!");
    }
}

fn get_blocksize(disk: &[i64], id: i64) -> usize {
    let pos = disk
        .iter()
        .position(|&x| x == id)
        .expect("invalid block id");
    let mut size = 0;

    for num in &disk[pos..] {
        if *num != id {
            break;
        }
        size += 1;
    }

    size
}

fn find_empty_space(disk: &[i64], size: usize) -> Option<usize> {
    let mut pos = 0;
    let mut space;

    while let Some(found) = disk[pos..].iter().position(|&x| x == -1) {
        pos += found;
        space = 0;
        for num in &disk[pos..] {
            if *num != -1 {
                break;
            }
            space += 1;
        }
        if space >= size {
            return Some(pos);
        }
        pos += space;
    }

    None
}

fn compress(disk: &mut [i64]) {
    let mut id = *disk
        .iter()
        .rfind(|&x| *x != -1)
        .expect("No non-empty space!");

    while id >= 0 {
        let pos = disk
            .iter()
            .position(|&x| x == id)
            .expect("id does not exist!");
        let size = get_blocksize(disk, id);
        let space = match find_empty_space(disk, size) {
            Some(found) => found,
            None => disk.len(),
        };

        if space < pos {
            // &disk[pos..(pos + size)].swap_with_slice(&mut disk[space..(space + size)]);
            let (first, second) = disk.split_at_mut(pos);
            first[space..(space + size)].swap_with_slice(&mut second[..size]);
        }
        print_disk(disk);

        id -= 1;
    }
}

fn calc_checksum(disk: &[i64]) -> i64 {
    let mut checksum = 0;

    for i in 0..disk.len() {
        if disk[i] == -1 {
            continue;
        }
        checksum += i as i64 * disk[i];
    }

    checksum
}

fn print_disk(disk: &[i64]) {
    for num in disk {
        match num {
            -1 => print!("."),
            _ => print!("[{num}]"),
            // _ => print!("{num}"),
        }
    }
    println!();
}

fn main() {
    // let input = get_input("test.txt");
    let input = get_input("input.txt");

    let mut disk = build_disk(input);

    // fragment(&mut disk);

    print_disk(&disk);
    compress(&mut disk);

    let checksum = calc_checksum(&disk);

    println!("checksum: {checksum}");
}
