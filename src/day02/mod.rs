#![allow(unused)]

use std::cmp;
use std::fs;

fn evaluate(state: &mut Vec<i64>)
{
    let mut pos = 0;

    loop
    {
        let opcode = state[pos];

        if opcode == 99
        {
            return
        }

        let index1 = state[pos+1] as usize;
        let index2 = state[pos+2] as usize;
        let index3 = state[pos+3] as usize;

        state[index3] = match opcode
        {
            1  => state[index1] + state[index2],
            2  => state[index1] * state[index2],
            _  => panic!("Invalid opcode!")
        };

        pos += 4;
    }
}

pub fn solve_part_one(input_file: &str, init: &[(usize, i64)]) -> i64
{
    let contents = fs::read_to_string(input_file)
        .expect(&format!("Unable to read file {}", input_file));

    let mut state = Vec::new();
    
    for token in contents.split(",")
    {
        state.push(token.parse::<i64>().unwrap());
    }

    for (index, value) in init
    {
        state[*index] = *value;
    }

    evaluate(&mut state);

    state[0]
}

pub fn solve_part_two(input_file: &str) -> i64
{
    let contents = fs::read_to_string(input_file)
        .expect(&format!("Unable to read file {}", input_file));

    0
}

#[cfg(test)]
mod examples
{
    use crate::day02::*;

    #[test]
    fn part_one()
    {
        let mut state = vec![1,0,0,0,99];
        evaluate(&mut state);
        assert_eq!(state, vec![2,0,0,0,99]);

        let mut state = vec![2,3,0,3,99];
        evaluate(&mut state);
        assert_eq!(state, vec![2,3,0,6,99]);

        let mut state = vec![2,4,4,5,99,0];
        evaluate(&mut state);
        assert_eq!(state, vec![2,4,4,5,99,9801]);

        let mut state = vec![1,1,1,4,99,5,6,0,99];
        evaluate(&mut state);
        assert_eq!(state, vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn part_two()
    {
    }
}