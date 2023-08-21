mod aoc1;
mod aoc2;

fn main() {
    println!(
        "AOC day 1, top elf: {}",
        aoc1::top_n_elves("data/aoc1.txt", 1)
    );
    println!(
        "AOC day 1, top 3 elves: {}",
        aoc1::top_n_elves("data/aoc1.txt", 3)
    );

    println!(
        "AOC day 2, total points (part 1): {}",
        aoc2::total_points("data/aoc2.txt", true)
    );
    println!(
        "AOC day 2, total points (part 2): {}",
        aoc2::total_points("data/aoc2.txt", false)
    );
}
