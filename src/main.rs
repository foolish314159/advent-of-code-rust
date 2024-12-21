mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;
mod aoc8;

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

    println!(
        "AOC day 5 (part 1): {}",
        aoc5::top_crates("data/aoc5.txt", false)
    );
    println!(
        "AOC day 5 (part 1): {}",
        aoc5::top_crates("data/aoc5.txt", true)
    );

    println!(
        "AOC day 6 (part 1): {}",
        aoc6::chars_before_marker::<4>("data/aoc6.txt")
    );
    println!(
        "AOC day 6 (part 2): {}",
        aoc6::chars_before_marker::<14>("data/aoc6.txt")
    );

    println!(
        "AOC day 7 (part 1): {}",
        aoc7::size_of_dirs_below("data/aoc7.txt", 100000)
    );
    println!(
        "AOC day 7 (part 2): {}",
        aoc7::dir_to_delete("data/aoc7.txt", 70000000, 30000000)
    );

    println!(
        "AOC day 8 (part 1): {}",
        aoc8::visible_trees("data/aoc8.txt")
    );
    println!(
        "AOC day 8 (part 2): {}",
        aoc8::highest_scenic_score("data/aoc8.txt")
    );
}
