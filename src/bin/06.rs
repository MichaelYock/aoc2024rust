advent_of_code::solution!(6);

//use std::fs;

pub fn part_one(input: &str) -> Option<u32> {
    //let test = fs::read_to_string("./data/examples/06.txt").unwrap();

    let mut grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
     
    let mut pos: (isize, isize) = (0,0);

    let directions: Vec<(isize, isize)> = vec![(-1,0),(0,1),(1,0),(0,-1)];
    let mut curr_direction = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '^' {
                pos = (r as isize,c as isize);
                break
            }
        }
    }

    let mut counter = 1;
    loop {
        let (r, c) = pos;
        let (dy,dx) = directions[curr_direction];
        let x = (r + dy) as usize;
        let y = (c + dx) as usize;
        if x < grid.len() && y < grid[0].len() {
            if grid[x][y] != '#' {
                if grid[x][y] != 'X' {
                    grid[x][y] = 'X';
                    counter += 1;
                }
                pos = (x as isize, y as isize);
            } else if curr_direction == 3 {
                
                curr_direction = 0;
        
            } else {
                curr_direction += 1;
            }
        } else {
            grid[r as usize][c as usize] = 'X';
            break
        }
    }

    for i in 0..grid.len() {
        println!("{:?}", grid[i]);
    }
    
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    //let test = fs::read_to_string("./data/examples/05.txt").unwrap();

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
