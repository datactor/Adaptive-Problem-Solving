use std::{
    error::Error,
    io::{self, BufWriter, Read, Write},
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_ascii_whitespace(),
        }
    }

    fn next<T>(&mut self) -> Result<T, Box<dyn Error>>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        self.input
            .next()
            .ok_or("EOF")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn ccw(&self, p1: &Point, p2: &Point) -> i32 {
        let cross_product = (p1.x - self.x) as i64 * (p2.y - self.y) as i64
            - (p1.y - self.y) as i64 * (p2.x - self.x) as i64;
        cross_product.signum() as i32
    }

    fn squared_distance_to(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        dx * dx + dy * dy
    }
}

struct Cvxh(Vec<Point>);

impl Cvxh {
    fn find_pivot(points: &mut Vec<Point>) -> Point {
        let min_idx = points
            .iter()
            .enumerate()
            .min_by_key(|&(_, point)| (point.x, point.y))
            .unwrap()
            .0;
        points.swap(0, min_idx);
        points[0]
    }

    fn sort_by_polar_angle(pivot: Point, points: &mut Vec<Point>) {
        points[1..].sort_unstable_by(|p1, p2| {
            let orientation = pivot.ccw(p1, p2);
            if orientation != 0 {
                0.cmp(&orientation)
            } else {
                pivot
                    .squared_distance_to(p1)
                    .cmp(&pivot.squared_distance_to(p2))
            }
        });
    }

    fn build_cvxh(points: &Vec<Point>) -> Vec<Point> {
        let mut cvxh: Vec<Point> = Vec::new();
        for &next_point in points {
            while cvxh.len() >= 2
                && cvxh[cvxh.len() - 2].ccw(cvxh.last().unwrap(), &next_point) <= 0
            {
                cvxh.pop();
            }
            cvxh.push(next_point);
        }
        cvxh
    }

    fn graham_scan_from_points(mut points: Vec<Point>) -> Vec<Point> {
        let pivot = Self::find_pivot(&mut points);
        Self::sort_by_polar_angle(pivot, &mut points);
        Self::build_cvxh(&points)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut writer = BufWriter::new(io::stdout());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let t = scanner.next::<usize>()?;
    for _ in 0..t {
        let n = scanner.next::<usize>()?;
        let points = (0..n)
            .map(|_| {
                let x = scanner.next::<i32>()?;
                let y = scanner.next::<i32>()?;
                Ok(Point { x, y })
            })
            .collect::<Result<Vec<Point>, Box<dyn Error>>>()?;
        let mut cvxh = Cvxh::graham_scan_from_points(points);
        cvxh.push(cvxh[0]);

        let mut perimeter = 0.0;
        for i in 0..cvxh.len() - 1 {
            perimeter += (cvxh[i].squared_distance_to(&cvxh[i + 1]) as f64).sqrt();
        }
        writeln!(writer, "{:.2}", perimeter)?;
    }

    Ok(())
}
