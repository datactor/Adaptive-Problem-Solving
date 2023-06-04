use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    cmp::Ordering,
};

const INF: f64 = f64::INFINITY;
const EPS: f64 = 1e-12;

#[derive(Clone, Eq, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
    p: i32,
    q: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        ((self.q * other.p - self.p * other.q) as f64).abs() < EPS && ((self.y - other.y) as f64).abs() < EPS && ((self.x - other.x) as f64).abs() < EPS
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        if ((self.q * other.p - self.p * other.q) as f64).abs() > EPS {
            Some(((self.q * other.p - self.p * other.q) as f64).partial_cmp(&0.0).unwrap())
        } else if ((self.y - other.y) as f64).abs() > EPS {
            Some(self.y.partial_cmp(&other.y).unwrap())
        } else {
            Some(self.x.partial_cmp(&other.x).unwrap())
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

macro_rules! read_input {
    ($reader:expr, $input:expr, $type:ty) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            $input.trim().parse::<$type>()
        }
    };
}

macro_rules! read_lines_to_vec {
    ($reader:expr, $input:expr, $num:expr, $type:ty) => {
        {
            let vec: Vec<Point> = (0..$num).filter_map(|_| {
                $input.clear();
                $reader.read_line(&mut $input).expect("Failed to read");
                let mut iter = $input.split_ascii_whitespace();
                let v1 = iter.next().expect("no 1st iter").parse::<$type>().expect("Failed to parse 1st val");
                let v2 = iter.next().expect("no 2nd iter").parse::<$type>().expect("Failed to parse 2nd val");
                Some(Point { x: v1, y: v2, p: 0, q: 0 })
            }).collect();
            vec
        }
    };
}

fn is_ccw(p1: &Point, p2: &Point, p3: &Point) -> bool {
    let a = (p2.x - p1.x) as i64 * (p3.y - p1.y) as i64;
    let b = (p2.y - p1.y) as i64 * (p3.x - p1.x) as i64;
    if a > b { return true }
    false
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();

    let mut t = 1;
    while let Ok(n) = read_input!(reader, buffer, usize) {
        if n == 0 { break }
        let mut points = read_lines_to_vec!(reader, buffer, n, i32);

        /// graham's scan
        // 1. preparation: zero setting (the lowest y value)
        let min_idx = points.iter().enumerate().min_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap()).unwrap().0;
        points.swap(0, min_idx);

        // 2. sorting: Sort all points in order of polar angle
        // This way, you only have to traverse all the points
        // once to find the points that form the convex hull.
        for i in 1..n {
            points[i].p = points[i].x - points[0].x;
            points[i].q = points[i].y - points[0].y;
        }
        points[1..].sort_unstable();

        // 3. scanning: While traversing the sorted points,
        // it is determined whether the current point can be included in the convex hull.
        // To do this, we check if the two most recently added points to the convex hull
        // and the current point are counterclockwise.
        // This is also known as a 'left turn' check.
        let mut hull_idx = Vec::new();
        hull_idx.push(0);
        hull_idx.push(1);
        for i in 2..n {
            while hull_idx.len() >= 2 {
                let snd = hull_idx.pop().unwrap();
                let fst = *hull_idx.last().expect("Empty");
                if is_ccw(&points[fst], &points[snd], &points[i]) {
                    hull_idx.push(snd);
                    break;
                }
            }
            hull_idx.push(i);
        }

        let hull: Vec<Point> = hull_idx.into_iter().map(|idx| points[idx]).collect();
        let n = hull.len();
        let mut ans = INF;

        for i in 0..n {
            let mut res: f64 = 0.0;
            if hull[i].x == hull[(i + 1) % n].x {
                for j in 0..n {
                    res = res.max((hull[i].x - hull[j].x).abs() as f64);
                }
                ans = ans.min(res);
                continue;
            }

            let a = ((hull[(i + 1) % n].y - hull[i].y) as f64) / ((hull[(i + 1) % n].x - hull[i].x) as f64);
            let b = -1.0;
            let c = hull[i].y as f64 - a * hull[i].x as f64;

            for j in 0..n {
                res = res.max((a * hull[j].x as f64 + b * hull[j].y as f64 + c).abs() / (a * a + b * b).sqrt());
            }
            ans = ans.min(res);
        }

        write!(writer, "Case {}: {:.2}\n", t, ans + 0.00499999999)?;
        t += 1;

    }

    Ok(())
}