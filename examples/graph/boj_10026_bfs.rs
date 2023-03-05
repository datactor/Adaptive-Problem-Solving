use std::{collections::VecDeque, io};

fn input() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn as_byte(c: char) -> u8 {
    match c {
        'R' => 1,
        'G' => 2,
        'B' => 3,
        _ => unreachable!(),
    }
}

fn bfs(table: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, n: usize, color: u8) -> i32 {
    let mut q = VecDeque::new();
    let mut cnt = 0;

    for i in 0..n {
        for j in 0..n {
            if table[i][j] == color && !visited[i][j] {
                q.push_back((j as i32, i as i32));
                visited[i][j] = true;
                cnt += 1;
            }

            while let Some((cur_x, cur_y)) = q.pop_front() {
                for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let next_x = cur_x + dx;
                    let next_y = cur_y + dy;

                    if next_x < 0 || next_x >= n as i32 || next_y < 0 || next_y >= n as i32 {
                        continue;
                    }

                    if visited[next_y as usize][next_x as usize] {
                        continue;
                    }

                    if table[next_y as usize][next_x as usize] == color {
                        q.push_back((next_x, next_y));
                        visited[next_y as usize][next_x as usize] = true;
                    }
                }
            }
        }
    }

    cnt
}

fn main() {
    let n = input()[0] as usize;

    let mut table = vec![vec![0; n]; n];
    let mut visited = vec![vec![false; n]; n];

    for i in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let mut chars = s.chars();

        for j in 0..n {
            table[i][j] = as_byte(chars.next().unwrap());
        }
    }

    print!(
        "{} ",
        bfs(&table, &mut visited, n, 1)
            + bfs(&table, &mut visited, n, 2)
            + bfs(&table, &mut visited, n, 3)
    );

    for i in 0..n {
        for j in 0..n {
            visited[i][j] = false;

            if table[i][j] == 2 {
                table[i][j] = 1;
            }
        }
    }

    print!(
        "{}",
        bfs(&table, &mut visited, n, 1)
            + bfs(&table, &mut visited, n, 3)
    );
}