mod day01;

fn main() 
{
    println!("Day 01 Part 1: {}", day01::solve_part_one("src/day01/input.txt"));
    println!("Day 01 Part 2: {}", day01::solve_part_two("src/day01/input.txt"));
}

#[cfg(test)]
mod answers
{
    #[test]

    fn day01()
    {
        use crate::day01::*;
        assert_eq!(solve_part_one("src/day01/input.txt"), 3427972);
        assert_eq!(solve_part_two("src/day01/input.txt"), 5139078);
    }
}
