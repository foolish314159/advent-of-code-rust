mod aoc1;

fn main() {
    println!(
        "AOC day 1, top elf: {}",
        aoc1::top_n_elves("data/aoc1.txt", 1)
    );
    println!(
        "AOC day 1, top 3 elves: {}",
        aoc1::top_n_elves("data/aoc1.txt", 3)
    );
}
