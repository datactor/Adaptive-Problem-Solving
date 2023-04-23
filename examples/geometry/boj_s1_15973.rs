// https://www.acmicpc.net/problem/15973

use std::{
    io::{self, prelude::*, BufWriter},
};

fn point(ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32, dx: i32, dy: i32) -> bool {
    (ax, ay) == (dx, dy) || (bx, by) == (cx, cy) || (bx, ay) == (cx, dy) || (ax, by) == (dx, cy)
}

fn line(ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32, dx: i32, dy: i32) -> bool {
    // 꼭짓점 하나 일치
    ((bx, by) == (cx, dy) && by > cy) || ((bx, ay) == (cx, cy) && dy > ay)
        || ((dx, cy) == (ax, ay) && by > cy) || ((dx, dy) == (ax, by) && dy > ay)
        // 변 일치
        || ((bx, by) == (cx, dy) && (bx, ay) == (cx, cy))
        || ((ax, ay) == (dx, cy) && (dx, dy) == (ax, by))
        || ((cx, cy) == (ax, by) && (bx, by) == (dx, cy))
        || ((ax, ay) == (cx, dy) && (dx, dy) == (bx, ay))
        // 그 사이(a,b의 한 변에 c,d가 겹침)
        || (ax == dx && (ay < cy && cy < by || ay < dy && dy < by))
        || (bx == cx && (ay < cy && cy < by || ay < dy && dy < by))
        || (by == cy && (ax < cx && cx < bx || ax < dx && dx < bx))
        || (ay == dy && (ax < cx && cx < bx || ax < dx && dx < bx))
        // 그 사이(c,d의 한 변에 a,b가 겹침)
        || (ax == dx && (cy < ay && ay < dy || cy < by && by < dy))
        || (bx == cx && (cy < ay && ay < dy || cy < by && by < dy))
        || (by == cy && (cx < ax && ax < dx || cx < bx && bx < dx))
        || (ay == dy && (cx < ax && ax < dx || cx < bx && bx < dx))
}

fn null(ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32, dx: i32, dy: i32) -> bool {
    if bx < cx || dx < ax || by < cy || dy < ay {
        return true;
    } else if bx - ax < dx - cx && by - ay < dy - cy && cx < ax && ax < dx && cx < bx && bx < dx && cy < ay && ay < dy && cy < by && by < dy {
        return true;
    } else if bx - ax > dx - cx && by - ay > dy - cy && ax < cx && cx < bx && ax < dx && dx < bx && ay < cy && cy < by && ay < dy && dy < by {
        return true;
    } else {
        return false;
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let mut iter = input.split_ascii_whitespace();
    let mut read = || iter.next().unwrap().parse::<i32>().unwrap();
    let (ax1, ay1, ax2, ay2) = (read(), read(), read(), read());
    let (bx1, by1, bx2, by2) = (read(), read(), read(), read());

    writeln!(output, "{}", if line(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) {
        "LINE"
    } else if point(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) {
        "POINT"
    } else if null(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) {
        "NULL"
    } else {
        "FACE"
    })?;

    Ok(())
}