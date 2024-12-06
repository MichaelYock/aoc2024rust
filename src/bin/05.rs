advent_of_code::solution!(5);

//use std::fs;
use itertools::Itertools;


pub fn part_one(input: &str) -> Option<u32> {
    //let test = fs::read_to_string("./data/examples/05.txt").unwrap();

    let lines: Vec<_> = input.lines().collect();

    let split: Vec<_> = lines.split(|line| line.is_empty()).collect();

    let rules: Vec<_> = split[0].iter().map(|line| {
        let mut split = line.split("|");
        (split.next().unwrap().parse::<usize>().unwrap(),split.next().unwrap().parse::<usize>().unwrap())
    }).collect();

    let updates: Vec<Vec<_>> = split[1].iter().map(|line| {
        line.split(",").map(|s| s.parse::<usize>().unwrap()).collect()
    }).collect();

    let sum: usize = updates
        .iter()
        .filter(|update| { !update.iter().combinations(2).map(|v| (v[0], v[1])).any(|(&x,&y)|
            rules.iter().any(|rule| rule.0 == y && rule.1 == x))
        })
        .map(|update| update[update.len()/2]).sum();

    Some(sum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {

    //let test = fs::read_to_string("./data/examples/05.txt").unwrap();

    let lines: Vec<_> = input.lines().collect();

    let split: Vec<_> = lines.split(|line| line.is_empty()).collect();

    let rules: Vec<_> = split[0].iter().map(|line| {
        let mut split = line.split("|");
        (split.next().unwrap().parse::<usize>().unwrap(),split.next().unwrap().parse::<usize>().unwrap())
    }).collect();

    let updates: Vec<Vec<_>> = split[1].iter().map(|line| {
        line.split(",").map(|s| s.parse::<usize>().unwrap()).collect()
    }).collect();

    let sum: usize = updates
        .iter()
        .filter(|update| { update.iter().combinations(2).map(|v| (v[0], v[1])).any(|(&x,&y)|
            rules.iter().any(|rule| rule.0 == y && rule.1 == x))
        })
        .map(|update| {
            let mut update = update.clone();

            while let Some ((i0, i1)) = update
                .iter()
                .combinations(2)
                .map(|v| (v[0], v[1]))
                .find_map(|(&x,&y)| {
                    rules.iter().find(|r| r.1 == x && r.0 == y).map(|r| {
                        (
                            update.iter().position(|&e| e == r.0).unwrap(),
                            update.iter().position(|&e| e == r.1).unwrap()
                        )
                    })
                }) 
                {
                    update.swap(i0, i1);
                }
                update

        })
        .map(|update| update[update.len()/2]).sum();

    Some(sum.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
