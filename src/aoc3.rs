use std::fs::read_to_string;

pub fn sum_priorities(filename: &str) -> u32 {
    let content = read_to_string(filename).unwrap_or(String::from(""));
    content.lines().fold(0u32, |sum, line| {
        let (half1, half2) = line.split_at(line.len() / 2);
        if let Some(dup_index) = line.find(|c: char| half1.contains(c) && half2.contains(c)) {
            sum + priority(&line.chars().nth(dup_index).unwrap())
        } else {
            sum
        }
    })
}

pub fn sum_group_priorities(filename: &str) -> u32 {
    groups(filename)
        .iter()
        .fold(0u32, |sum, group| sum + priority(&badge(&group)))
}

fn badge(group: &Vec<String>) -> char {
    for c in group[0].chars() {
        if group[1].contains(c) && group[2].contains(c) {
            return c;
        }
    }

    panic!("Something went wrong, no badge found in group");
}

fn groups(filename: &str) -> Vec<Vec<String>> {
    let content = read_to_string(filename).unwrap_or(String::from(""));
    content.lines().fold(vec![vec![]], |mut groups, line| {
        if groups.last().unwrap().len() == 3 {
            groups.push(vec![]);
        }
        groups.last_mut().unwrap().push(line.to_string());
        groups
    })
}

fn priority(c: &char) -> u32 {
    if c.is_lowercase() {
        *c as u32 - 96
    } else {
        *c as u32 - 38
    }
}
