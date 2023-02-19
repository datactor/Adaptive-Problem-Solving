use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();
    let n = lines.next().unwrap().trim().parse::<i32>().unwrap();

    let mut point = (0, 0);
    let mut points = Vec::new();
    let mut x = Vec::new();
    let mut y = Vec::new();
    for _ in 0..6 {
        let mut v = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>())
            .flatten();
        let dir = v.next().unwrap();
        let dst = v.next().unwrap();
        match dir {
            1 => {
                point.0 += dst;
                x.push(point.0);
                y.push(point.1);
                points.push(point);
            }
            2 => {
                point.0 -= dst;
                x.push(point.0);
                y.push(point.1);
                points.push(point);
            }
            3 => {
                point.1 -= dst;
                x.push(point.0);
                y.push(point.1);
                points.push(point);
            }
            _ => {
                point.1 += dst;
                x.push(point.0);
                y.push(point.1);
                points.push(point);
            }
        }
    }

    let x_max = x.iter().max().unwrap();
    let x_min = x.iter().min().unwrap();
    let y_max = y.iter().max().unwrap();
    let y_min = y.iter().min().unwrap();

    let mut area = (x_max - x_min) * (y_max - y_min);

    let quad = [
        (*x_max, *y_max),
        (*x_min, *y_max),
        (*x_min, *y_min),
        (*x_max, *y_min),
    ];
    // let fst_quad = (*x_max, *y_max);
    // let snd_quad = (*x_min, *y_max);
    // let trd_quad = (*x_min, *y_min);
    // let fth_quad = (*x_max, *y_min);

    let (mut orphan_x, mut orphan_y) = (0, 0);
    for i in quad {
        if !points.contains(&i) {
            (orphan_x, orphan_y) = i;
        }
    }

    let mut closest_x = 500;
    let mut closest_y = 500;
    for (x, y) in points {
        if x == orphan_x {
            closest_y = (orphan_y - y).abs().min(closest_y);
        }
        if y == orphan_y {
            closest_x = (orphan_x - x).abs().min(closest_x);
        }
    }

    writeln!(output, "{}", (area - closest_x * closest_y) * n).unwrap();
}
