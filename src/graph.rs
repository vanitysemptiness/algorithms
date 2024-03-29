/*
graph algorithms
 */
#[allow(dead_code)]
pub mod graph {
    use std::collections::HashSet;

    use rand::{ thread_rng, seq::SliceRandom };

    const OBSTICLE: i32 = 1;
    const EMPTY: i32 = 0;

    
    fn generate_valid_grid(w: i32, h: i32) -> Vec<Vec<i32>> {
        let _rng = rand::thread_rng();
        // 2d array of random 0s and 1s
        let mut grid = vec![vec![1; w as usize]; h as usize];
        for i in 0..h {
            for j in 0..w {
                if i == j {
                    grid[i as usize][j as usize] = 0;
                }
            }
        }
        let _start = (0, 0);
        let _end = (h - 1, w - 1);
        // Generate random path
        // generate_valid_path(&mut grid, start, end);

        grid
    }

    fn generate_valid_path(grid: &mut Vec<Vec<i32>>, start: &(i32, i32), end: &(i32, i32)) {
        // set valid start and end
        grid[start.0 as usize][start.1 as usize] = 0;
        grid[end.0 as usize][end.1 as usize] = 0;

        let mut moves = vec![
            (-1, -1), // up left
            (-1, 0), // up
            (-1, 1), // up right
            (0, -1), // left
            (0, 1), // right
            (1, -1), // down left
            (1, 0), // down
            (1, 1) // down right
        ];
        let mut rng = thread_rng();
        let mut current: (i32, i32) = *start;
        /*
        we want to know if it is in bounds and it decreases the euclidian distance
         */
        while current != *end {
            moves.shuffle(&mut rng);
            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            for (x_curr, y_curr) in &moves {
                let next: (i32, i32) = ((current.0 as i32) + x_curr, (current.1 as i32) + y_curr);
                // if its been visited skip it
                if visited.contains(&next) {
                    continue;
                }
                // if its in bounds and is closer to the end, move forward
                if
                    is_valid_next_move(&grid, &current, &next, &end)
                {
                    grid[next.0 as usize][next.1 as usize] = 0;
                    visited.insert(next);
                    current = next;
                    break;
                }
            }
        }
    }

    fn is_obsticle(grid: &Vec<Vec<i32>>, next: &(i32, i32)) -> bool {
        grid[next.0 as usize][next.1 as usize] == OBSTICLE
    }

    fn is_closer(current: &(i32, i32), next: &(i32, i32), end: &(i32, i32)) -> bool {
        let next_dist = euclidian_distance(next, end);
        let curr_dist = euclidian_distance(current, end);
        next_dist < curr_dist
    }

    fn euclidian_distance(next: &(i32, i32), end: &(i32, i32)) -> i32 {
        let x = (end.0 - (next.0 as i32)).pow(2);
        let y = (end.1 - (next.1 as i32)).pow(2);
        let distance = (x + y) as f64;
        distance.sqrt().floor() as i32
    }

    fn is_in_bounds(grid: &Vec<Vec<i32>>, next: &(i32, i32)) -> bool {
        let h = grid.len() as i32;
        let w = grid[0].len() as i32;
        next.0 >= 0 && next.0 < h && next.1 >= 0 && next.1 < w
    }

    fn is_valid_next_move(
        grid: &Vec<Vec<i32>>,
        current: &(i32, i32),
        next: &(i32, i32),
        end: &(i32, i32)
    ) -> bool {
        // if its in bounds and is closer to the end, move forward
        return 
            is_in_bounds(&grid, &next) &&
            is_closer(&current, &next, &end) &&
            !is_obsticle(&grid, &next);
    }


    pub mod traversals {
        static MOVES: [(i32, i32); 8] = [
            (-1, -1), // up left
            (-1, 0), // up
            (-1, 1), // up right
            (0, -1), // left
            (0, 1), // right
            (1, -1), // down left
            (1, 0), // down
            (1, 1), // down right
        ];

        use std::collections::{ VecDeque, HashSet, BinaryHeap };

        use super::{ is_valid_next_move, euclidian_distance };

        pub fn bfs(grid: &Vec<Vec<i32>>, start: &(i32, i32), end: &(i32, i32)) -> bool {
            let mut q = VecDeque::new(); // queue
            let mut visited: HashSet<(i32, i32)> = HashSet::new();

            q.push_back(*start);
            visited.insert(*start);
            while !q.is_empty() {
                let current = q.pop_front().unwrap();
                // we have reached the end
                if current == *end {
                    return true;
                }

                for neighbor in MOVES.iter() {
                    let next: (i32, i32) = (current.0 + neighbor.0, current.1 + neighbor.1);
                    if
                    is_valid_next_move(&grid, &current, &next, &end)
                    {
                        q.push_back(next);
                        visited.insert(next);
                    }
                }
            }
            // unable to reach the end
            return false;
        }

        pub fn dfs(grid: &Vec<Vec<i32>>, start: &(i32, i32), end: &(i32, i32)) -> bool {
            let mut stack = Vec::new(); //stack
            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            stack.push(*start);
            visited.insert(*start);
            while !stack.is_empty() {
                let current = stack.pop().unwrap();
                // we have reached the end
                if current == *end {
                    return true;
                }
                let (x_curr, y_curr) = (current.0, current.1);
                for (x_move, y_move) in &MOVES {
                    let next: (i32, i32) = (x_curr + x_move, y_curr + y_move);
                    if
                        is_valid_next_move(&grid, &current, &next, &end)
                    {
                        stack.push(next);
                        visited.insert(next);
                    }
                }
            }
            return false;
        }

        pub fn a_star(grid: &Vec<Vec<i32>>, start: &(i32, i32), end: &(i32, i32)) -> bool {
            let mut heap: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            heap.push((0, start.0, start.1)); // doesn't matter the first euclidian value
            visited.insert(*start);
            while !heap.is_empty() {
                let current = heap.pop().unwrap();
                // we have reached the end
                if (current.1, current.2) == *end {
                    return true;
                }
                let (x_curr, y_curr) = (current.1, current.2);
                for (x_move, y_move) in &MOVES {
                    let next: (i32, i32) = (x_curr + x_move, y_curr + y_move);
                    if
                        is_valid_next_move(&grid, &(current.1, current.2), &next, &end)
                    {
                        heap.push((-1 * euclidian_distance(&next, &end), next.0, next.1));
                        visited.insert(next);
                    }
                }
            }
            return false;
        }
    }

    #[cfg(test)]
    mod graph_test {
        use std::time::Instant;

        use crate::timer::timer::report_function_timing;

        use super::*;

        #[test]
        fn test_generate_grid() {
            let start = Instant::now();
            generate_valid_grid(10000, 10000);
            report_function_timing(start, "generate grid");
        }

        fn print_grid(grid: &Vec<Vec<i32>>) {
            for row in grid {
                println!("{:?}", row);
            }
        }

        #[test]
        fn test_dfs() {
            let grid = generate_valid_grid(10000, 10000);
            let s = (0, 0);
            let end = ((grid.len() as i32) - 1, (grid[0].len() as i32) - 1);
            let start = Instant::now();
            let result = traversals::dfs(&grid, &s, &end);
            report_function_timing(start, "dfs grid");
            assert_eq!(result, true);
        }

        #[test]
        fn test_bfs() {
            let grid = generate_valid_grid(10000, 10000);
            let s = (0, 0);
            let end = ((grid.len() as i32) - 1, (grid[0].len() as i32) - 1);
            let start = Instant::now();
            let result = traversals::bfs(&grid, &s, &end);
            report_function_timing(start, "bfs grid");
            assert_eq!(result, true);
        }

        #[test]
        fn test_a_star() {
            let grid = generate_valid_grid(10000, 10000);
            let s = (0, 0);
            let end = ((grid.len() as i32) - 1, (grid[0].len() as i32) - 1);
            let start = Instant::now();
            let result = traversals::a_star(&grid, &s, &end);
            report_function_timing(start, "A* grid");
            assert_eq!(result, true);
        }
    }
}
