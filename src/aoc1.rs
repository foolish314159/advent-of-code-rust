use itertools::Itertools;
use std::fs::read_to_string;

/// Return the total amount of calories of the top n elves carrying the most calories
///
/// # Arguments
///
/// * `filename` - Path to input file containing a list of calories each elf is carrying.
///                Each line contains one integer corresponding to an amount of calories.
///                An empty line in indicates the end for one elf and the next line will
///                belong to the next elf.
///
/// * `n`        - Number of elves
pub fn top_n_elves(filename: &str, n: usize) -> u32 {
    parse_elves(filename)
        .iter()
        .map(|elf| elf.iter().sum::<u32>())
        // .sorted_desc() see below
        .sorted()
        .rev()
        .take(n)
        .sum::<u32>()
}

fn parse_elves(filename: &str) -> Vec<Vec<u32>> {
    let content = read_to_string(filename).unwrap_or(String::from(""));
    content.lines().fold(vec![vec![]], |mut elves, line| {
        if line.is_empty() {
            elves.push(vec![]);
        } else if let Some(elf) = elves.last_mut() {
            elf.push(line.parse::<u32>().unwrap_or(0));
        }
        elves
    })
}

// trait SortedDescIterator: Iterator {
//     fn sorted_desc(self) -> IntoIter<Self::Item>
//     where
//         Self::Item: Ord;
// }

// impl<I> SortedDescIterator for I
// where
//     I: Iterator,
// {
//     fn sorted_desc(self) -> IntoIter<Self::Item>
//     where
//         Self::Item: Ord,
//     {
//         self.sorted().rev().collect::<Vec<_>>().into_iter()
//     }
// }
