use std::cmp;
use std::fs;

fn naive_fuel_required(mass: i64) -> i64
{
    cmp::max(mass / 3 - 2, 0)
}

fn fuel_required(mut mass: i64) -> i64
{
    let mut fuel = 0;

    while mass > 0
    {
        mass = naive_fuel_required(mass);
        fuel += mass;
    }

    fuel
}

pub fn solve_part_one(input_file: &str) -> i64
{
    let contents = fs::read_to_string(input_file)
        .expect(&format!("Unable to read file {}", input_file));

    let mut total_fuel = 0;

    for line in contents.split("\n")
    {
        let mass = line.parse::<i64>().unwrap();
        let fuel = naive_fuel_required(mass);

        total_fuel += fuel;
    }

    total_fuel
}

pub fn solve_part_two(input_file: &str) -> i64
{
    let contents = fs::read_to_string(input_file)
        .expect(&format!("Unable to read file {}", input_file));

    let mut total_fuel = 0;

    for line in contents.split("\n")
    {
        let mass = line.parse::<i64>().unwrap();
        let fuel = fuel_required(mass);

        total_fuel += fuel;
    }

    total_fuel
}

#[cfg(test)]
mod examples
{
    use crate::day01::*;

    #[test]
    fn part_one()
    {
        assert_eq!(naive_fuel_required(    12),     2);
        assert_eq!(naive_fuel_required(    14),     2);
        assert_eq!(naive_fuel_required(  1969),   654);
        assert_eq!(naive_fuel_required(100756), 33583);
    }

    #[test]
    fn part_two()
    {
        assert_eq!(fuel_required(    14),     2);
        assert_eq!(fuel_required(  1969),   966);
        assert_eq!(fuel_required(100756), 50346);
    }
}