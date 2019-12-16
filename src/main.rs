mod day01;
mod day02;

fn main() 
{
    // println!("Day 01 Part 1: {}", day01::solve_part_one("src/day01/input.txt"));
    // println!("Day 01 Part 2: {}", day01::solve_part_two("src/day01/input.txt"));

    println!("Day 02 Part 1: {}", day02::solve_part_one("src/day02/input.txt", &[(1, 12), (2, 2)]));
    // println!("Day 02 Part 2: {}", day02::solve_part_two("src/day02/input.txt"));
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

    #[test]
    fn day02()
    {
        use crate::day02::*;
        assert_eq!(solve_part_one("src/day02/input.txt", &[(1, 12), (2, 2)]), 7594646);
        // assert_eq!(solve_part_two("src/day01/input.txt"), 5139078);
    }
}
