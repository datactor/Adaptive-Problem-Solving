use std::{
    io::{self, BufRead, Write},
};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i32 {
    let res = (p2.x - p1.x)*(p3.y - p1.y) - (p3.x - p1.x)*(p2.y - p1.y);
    if res > 0 { 1 }
    else if res < 0 { -1 }
    else { 0 }
}

struct Line(Point, Point);

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Line(Point::min(p1, p2), Point::max(p1, p2))
    }

    fn intersect_with(&self, other: &Self) -> bool {
        match (
            ccw(&self.0, &self.1, &other.0) * ccw(&self.0, &self.1, &other.1),
            ccw(&other.0, &other.1, &self.0) * ccw(&other.0, &other.1, &self.1)
        ) {
            (0, 0) => other.0 <= self.1 && self.0 <= other.1,
            (p, q) => p <= 0 && q <= 0,
        }
    }
}

struct Square(Point, Point);

impl Square {
    fn new(p1: &Point, p2: &Point) -> Self {
        Square(
            Point::new(i32::min(p1.x, p2.x), i32::min(p1.y, p2.y)),
            Point::new(i32::max(p1.x, p2.x), i32::max(p1.y, p2.y)),
        )
    }

    fn includes(&self, point: &Point) -> bool {
        (self.0.x <= point.x && point.x <= self.1.x) && (self.0.y <= point.y && point.y <= self.1.y)
    }

    fn intersect_with(&self, line: &Line) -> bool {
        if self.includes(&line.0) || self.includes(&line.1) {
            return true;
        }

        let points = [
            Point::new(self.0.x, self.0.y),
            Point::new(self.1.x, self.0.y),
            Point::new(self.1.x, self.1.y),
            Point::new(self.0.x, self.1.y),
        ];

        for i in 0..4 {
            if Line::new(points[i], points[(i + 1) % 4]).intersect_with(&line) {
                return true;
            }
        }
        false
    }
}

fn main() -> io::Result<()> {
    let mut reader = io::BufReader::new(io::stdin().lock());
    let mut writer = io::BufWriter::new(io::stdout().lock());

    let mut input = String::new();
    reader.read_line(&mut input)?;

    let t = input.trim().parse::<usize>().unwrap();

    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input)?;
        let nums: Vec<i32> = input.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

        let intersects = Square::new(
            &Point::new(nums[4], nums[5]),
            &Point::new(nums[6], nums[7])
        ).intersect_with(&Line(
            Point::new(nums[0], nums[1]),
            Point::new(nums[2], nums[3]))
        );

        writeln!(writer, "{}", if intersects { "T" } else { "F" })?;
    }

    Ok(())
}