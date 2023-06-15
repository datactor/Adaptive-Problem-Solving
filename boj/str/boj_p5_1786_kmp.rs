// https://www.acmicpc.net/problem/1786

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    collections::BTreeSet,
};

macro_rules! read {
    ($reader:expr, $input:expr) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            let tmp = $input.as_bytes();
            tmp[..tmp.len()-1].to_vec()
        }
    }
}

// failed
// fn adderal(t: &Vec<u8>, p: &Vec<u8>, ti: usize) -> bool {
//     let iter = t[ti..ti+p.len()].iter().zip(p[0..p.len()].iter());
//     for (a, b) in iter {
//         if a != b {
//             return false
//         }
//     }
//     true
//     // if t[ti..ti+p.len()] != p[0..p.len()] {
//     //     false
//     // } else {
//     //     true
//     // }
// }
//
// fn main() -> io::Result<()> {
//     let mut read_buf = BufReader::new(io::stdin().lock());
//     let mut write_buf = BufWriter::new(io::stdout().lock());
//     let mut buf_to_string = String::new();
//
//     let t = read!(read_buf, buf_to_string);
//     let p = read!(read_buf, buf_to_string);
//
//     // let tlen = t.len();
//     // let plen = p.len();
//     // let mut ti = 0;
//     // let mut pi = 0;
//     // let mut vec = Vec::new();
//     // let mut bts = BTreeSet::new();
//     // while ti < tlen {
//     //     if pi >= plen {
//     //         // cnt += 1;
//     //         pi = 0;
//     //         // println!("{}", ti-plen+1);
//     //         vec.push(ti-plen+1);
//     //         bts.insert(ti-plen+1);
//     //         ti += 1;
//     //         pi += 1;
//     //         continue
//     //     }
//     //     // println!("{}: {} {}: {}", ti, t[ti], pi, p[pi]);
//     //
//     //     if t[ti] == p[0] && ti+plen < tlen {
//     //         if adderal(&t, &p, ti) {
//     //             // cnt += 1;
//     //             vec.push(ti+1);
//     //             bts.insert(ti+1);
//     //         }
//     //     }
//     //
//     //     if t[ti] == p[pi] {
//     //         ti += 1;
//     //         pi += 1;
//     //     } else {
//     //         if t[ti] == p[0] {
//     //             ti += 1;
//     //             pi = 1;
//     //         } else {
//     //             ti += 1;
//     //             pi = 0;
//     //         }
//     //     }
//     // }
//     // vec.dedup();
//     //
//     // writeln!(write_buf, "{}", bts.len())?;
//     // for i in bts {
//     //     write!(write_buf, "{} ", i)?;
//     // }
//
//     Ok(())
// }

macro_rules! ok {
    (()) => {
        {
            let mut read_buf = BufReader::new(io::stdin().lock());
            let mut write_buf = BufWriter::new(io::stdout().lock());
            let mut buf_to_string = String::new();

            let t = read!(read_buf, buf_to_string);
            let p = read!(read_buf, buf_to_string);
            let mut table = vec![0; p.len()];
            let mut indices = Vec::new();

            let mut j = 0;
            for i in 1..p.len() {
                while j > 0 && p[i] != p[j] {
                    j = table[j - 1];
                }
                if p[i] == p[j] {
                    j += 1;
                    table[i] = j;
                }
            }

            j = 0;
            for i in 0..t.len() {
                while j > 0 && t[i] != p[j] {
                    j = table[j - 1];
                }
                if t[i] == p[j] {
                    if j == p.len() - 1 {
                        indices.push(i+2-p.len());
                        j = table[j];
                    } else {
                        j += 1;
                    }
                }
            }

            writeln!(write_buf, "{}", indices.len())?;
            for &w in &indices {
                write!(write_buf, "{} ", w)?;
            }

            Ok(())
        }
    }
}

fn main() -> io::Result<()> {
    ok!(())
}