use std::fs;

enum ParseState {
    Rules,
    Updates,
}

fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to open input.")
}

fn parse_rule(line: &str) -> (i64, i64) {
    let (first, last) = line.split_once('|').expect("Failed to parse rule");
    let mut out = (0, 0);

    out.0 = first.parse().expect("Failed to parse first rule number");
    out.1 = last.parse().expect("Failed to parse second rule number");

    out
}

fn parse_update(line: &str) -> Vec<i64> {
    let mut update: Vec<i64> = Vec::new();

    for num in line.split(",") {
        update.push(num.parse().expect("Failed to parse update number"));
    }

    update
}

fn check_rule_applies(rule: (i64, i64), update: &[i64]) -> bool {
    update.contains(&rule.0) && update.contains(&rule.1)
}

fn update_follows_rule(rule: &(i64, i64), update: &[i64]) -> bool {
    let first_pos = update
        .iter()
        .position(|&x| x == rule.0)
        .expect("Rule does not apply to this update!");
    let second_pos = update
        .iter()
        .position(|&x| x == rule.1)
        .expect("Rule does not apply to this update!");

    first_pos < second_pos
}

fn update_follows_all_rules(rules: &[&(i64, i64)], update: &[i64]) -> bool {
    let mut follows_rules = true;

    for rule in rules {
        match update_follows_rule(rule, update) {
            true => (),
            false => {
                follows_rules = false;
                break;
            }
        }
    }

    follows_rules
}

fn fix_order(rules: &[&(i64, i64)], update: &mut [i64]) {
    while !update_follows_all_rules(rules, update) {
        for rule in rules {
            if !update_follows_rule(rule, update) {
                let first_pos = update
                    .iter()
                    .position(|&x| x == rule.0)
                    .expect("Rule does not apply to this update!");
                let second_pos = update
                    .iter()
                    .position(|&x| x == rule.1)
                    .expect("Rule does not apply to this update!");
                update.swap(first_pos, second_pos);
            }
        }
    }
}

fn main() {
    // let input = get_input("test.txt");
    let input = get_input("input.txt");

    let mut rules: Vec<(i64, i64)> = Vec::new();
    let mut updates: Vec<Vec<i64>> = Vec::new();
    let mut state = ParseState::Rules;

    // println!("{input}");

    for line in input.lines() {
        if line.is_empty() {
            state = ParseState::Updates;
            continue;
        }
        match state {
            ParseState::Rules => rules.push(parse_rule(line)),
            ParseState::Updates => updates.push(parse_update(line)),
        }
    }

    let mut answer = 0;
    let mut answer_p2 = 0;

    for mut update in updates {
        let relevant_rules: Vec<&(i64, i64)> = rules
            .iter()
            .filter(|&x| check_rule_applies(*x, &update))
            .collect();
        match update_follows_all_rules(&relevant_rules[..], &update) {
            true => {
                answer += update[update.len() / 2];
                println!("Follows rules! :D")
            }
            false => {
                fix_order(&relevant_rules, &mut update);
                answer_p2 += update[update.len() / 2];
                println!("Doesn't follow rules :(")
            }
        }
    }

    println!("answer: {answer}");
    println!("answer_p2: {answer_p2}");
}
