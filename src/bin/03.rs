advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    //let test = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let re = Regex::new(r"mul\((?<op1>[0-9]{1,3}),(?<op2>[0-9]{1,3})\)").unwrap();

    let sum = re.captures_iter(input).map(|cap| cap["op1"].parse::<u32>().unwrap() * cap["op2"].parse::<u32>().unwrap()).sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    //let test = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let mut active = true;

    let re: Regex = Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|(mul\((?<op1>[0-9]{1,3}),(?<op2>[0-9]{1,3})\))").unwrap();

    let mut sum = 0;
    
    for cap in re.captures_iter (input) {
        if let Some(_) = cap.name("do") { active = true }

        else if let Some(_) = cap.name("dont") { active = false }

        else if let (Some(op1), Some(op2)) = (cap.name("op1"), cap.name("op2")) {
            if active {
                let op1: u32 = op1.as_str().parse().unwrap();
                let op2: u32 = op2.as_str().parse().unwrap();

                sum += op1 * op2;
            }
        }
    }

    Some(sum)
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
