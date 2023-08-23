mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;

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

    println!(
        "AOC day 3 (part 1): {}",
        aoc3::sum_priorities("data/aoc3.txt")
    );
    println!(
        "AOC day 3 (part 2): {}",
        aoc3::sum_group_priorities("data/aoc3.txt")
    );

    println!(
        "AOC day 4 (part 1): {}",
        aoc4::sum_full_overlaps("data/aoc4.txt")
    );
    println!(
        "AOC day 4 (part 2): {}",
        aoc4::sum_partial_overlaps("data/aoc4.txt")
    );
}
