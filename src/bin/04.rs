use advent_of_code::VecTExt;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let test = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mas: Vec<_> = "MAS".chars().enumerate().map(|(i,c)|((i + 1) as isize, c)).collect();

    let directions = vec![(1,1),(1,0),(1,-1),(0,1),(0,-1),(-1,1),(-1,0),(-1,-1)];

    let mut counter = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'X' {
                let (r,c) = (r as isize, c as isize);

                for (dr, dc) in directions.iter() {
                    if mas.iter().all(|(step, char)|{
                        let x = r + dr * step;
                        let y = c + dc * step;

                        grid.iget(x).and_then(|e| e.iget(y)) == Some(char)
                    }) {

                        counter += 1;
                    }
                }
            }
        }
    }

    Some(counter)
    
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
