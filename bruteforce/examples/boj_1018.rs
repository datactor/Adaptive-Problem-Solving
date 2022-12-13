use std::io::{prelude::*, self};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let mut n = buffer.split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();
    let (n, m) = (n.next().unwrap(), n.next().unwrap());
    buffer.clear();

    io::stdin().read_to_string(&mut buffer).unwrap();
    let v = buffer.split_ascii_whitespace().map(
        |s| s.as_bytes()).collect::<Vec<_>>();

    let mut min = 32;

    for x in 0..n-7 {
        for y in 0..m-7 {
            let mut tmp1 = 0;
            let mut tmp2 = 0;
            for i in x..x+8 {
                for j in y..y+8 {
                    if (i+j)%2 == 0 {
                        if v[i][j] == 87 {
                            tmp2 += 1;
                            continue
                        } tmp1 += 1;
                    } else {
                        if v[i][j] == 66 {
                            tmp2 += 1;
                            continue
                        } tmp1 += 1;
                    }
                }
            }
            min = min.min(tmp1.min(tmp2));
        }
    }
    println!("{}", min);
}