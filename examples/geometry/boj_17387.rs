// https://www.acmicpc.net/problem/17387

use std::{
    io::{self, prelude::*},
    error::Error,
    cmp::{min, max},
    process::exit,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Scanner {
        Scanner {
            input: s.split_ascii_whitespace(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut scanner = Scanner::new(&input);

    let (x1, y1, x2, y2) = (scanner.read::<i128>(), scanner.read::<i128>(), scanner.read::<i128>(), scanner.read::<i128>());
    let (x3, y3, x4, y4) = (scanner.read::<i128>(), scanner.read::<i128>(), scanner.read::<i128>(), scanner.read::<i128>());

    let (min_l1_x, min_l1_y, max_l1_x, max_l1_y) = (min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2));
    let (min_l2_x, min_l2_y, max_l2_x, max_l2_y) = (min(x3, x4), min(y3, y4), max(x3, x4), max(y3, y4));

    // ccw 알고리즘으로 선분 l1과 l2의 두 끝점, 선분 l2와 l1의 두 끝점들의 위치관계를 탐색
    // return값이 0이면 평행, 양수일 경우 점은 선분의 반시계 방향에 위치, 음수일 경우 점은 선분의 시계방향에 위치
    let ccw123 = ccw(x1, y1, x2, y2, x3, y3);
    let ccw124 = ccw(x1, y1, x2, y2, x4, y4);
    let ccw341 = ccw(x3, y3, x4, y4, x1, y1);
    let ccw342 = ccw(x3, y3, x4, y4, x2, y2);

    if ccw123 * ccw124 == 0 && ccw341 * ccw342 == 0 { // 평행할 경우
        if min_l1_x <= max_l2_x && min_l2_x <= max_l1_x && min_l1_y <= max_l2_y && min_l2_y <= max_l1_y { // 평행하면서 겹치는 경우
            println!("1");
            exit(0);
        }
    } else {
        if ccw123 * ccw124 <= 0 && ccw341 * ccw342 <= 0 { // 일반적으로 교차될 경우
            println!("1");
            exit(0);
        }
    }

    println!("0");
    Ok(())
}


fn ccw(x1: i128, y1: i128, x2: i128, y2: i128, x3: i128, y3: i128) -> i128 {
    return (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1)
}