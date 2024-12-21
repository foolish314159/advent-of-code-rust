use std::fs::read_to_string;

pub fn visible_trees(filename: &str) -> u32 {
    let forest = parse_forest(filename);

    let mut visible = 0u32;

    for (i, row) in forest.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_visible(&forest, i, j) {
                visible += 1;
            }
        }
    }

    visible
}

pub fn highest_scenic_score(filename: &str) -> usize {
    let forest = parse_forest(filename);

    let mut highest = 0;

    for (i, row) in forest.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let score = scenic_score(&forest, i, j);
            if score > highest {
                highest = score;
            }
        }
    }

    highest
}

fn scenic_score(forest: &Vec<Vec<u32>>, row: usize, column: usize) -> usize {
    if row == 0 || row == forest.len() - 1 || column == 0 || column == forest.len() - 1 {
        return 0;
    }

    let height = forest[row][column];

    let score_left = forest[row][0..column]
        .iter()
        .rev() // reverse to start counting from center outwards
        .position(|tree| tree >= &height) // count trees until view is blocked
        .unwrap_or(column - 1) // or distance to left edge otherwise
        + 1; // position = 0 based index

    let score_right = forest[row][column + 1..forest[row].len()]
        .iter()
        .position(|tree| tree >= &height) // count trees until view is blocked
        .unwrap_or(forest[row].len() - column - 2) // or distance to right edge otherwise
        + 1; // position = 0 based index

    let score_top = forest[0..row]
        .iter()
        .rev() // reverse to start counting from center outwards
        .position(|row| row[column] >= height) // count trees until view is blocked
        .unwrap_or(row - 1) // or distance to top edge otherwise
        + 1; // position = 0 based index

    let score_bottom = forest[row + 1..forest.len()]
        .iter()
        .position(|row| row[column] >= height) // count trees until view is blocked
        .unwrap_or(forest.len() - row - 2) // or distance to bottom edge otherwise
        + 1; // position = 0 based index

    score_left * score_right * score_top * score_bottom
}

fn is_visible(forest: &Vec<Vec<u32>>, row: usize, column: usize) -> bool {
    if row == 0 || row == forest.len() - 1 || column == 0 || column == forest.len() - 1 {
        return true;
    }

    let height = forest[row][column];

    let hidden_left = forest[row][0..column].iter().any(|tree| tree >= &height);
    let hidden_right = forest[row][column + 1..forest[row].len()]
        .iter()
        .any(|tree| tree >= &height);

    let hidden_top = forest[0..row].iter().any(|row| row[column] >= height);
    let hidden_bottom = forest[row + 1..forest.len()]
        .iter()
        .any(|row| row[column] >= height);

    !hidden_left || !hidden_right || !hidden_top || !hidden_bottom
}

fn parse_forest(filename: &str) -> Vec<Vec<u32>> {
    let content = read_to_string(filename).unwrap_or(String::from(""));

    content
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
