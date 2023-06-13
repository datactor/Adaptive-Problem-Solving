use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    collections::{HashMap, BTreeMap},
};

macro_rules! read {
    ($reader:expr, $input:expr) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            $input.trim().to_string()
        }
    }
}

// macro_rules! ok {
//     (()) => {
//         {
//             let mut read_buf = BufReader::new(io::stdin().lock());
//             let mut write_buf = BufWriter::new(io::stdout().lock());
//             let mut buf_to_string = String::new();
//
//             let n = read!(read_buf, buf_to_string).parse::<usize>().unwrap();
//
//             let mut btm = BTreeMap::new();
//             for _ in 0..n {
//                 let key = read!(read_buf, buf_to_string);
//                 *btm.entry(key).or_insert(0) += 1;
//             }
//
//             // 백준의 Rust version에서는 last_key_value, pop_last가 unstable임
//             let max = btm.last_key_value().unwrap_or((&"none".to_string(), &0)).1.to_owned();
//             let mut most_popular = Vec::new();
//             while let Some(last) = btm.pop_last() {
//                 if last.1 == max {
//                     most_popular.push(last.0);
//                 } else {
//                     break
//                 }
//             }
//
//             most_popular.sort_unstable();
//
//             if let Some(book) = most_popular.first() {
//                 write!(write_buf, "{}", book)?;
//             }
//
//             Ok(())
//         }
//     }
// }

macro_rules! ok {
    (()) => {
        {
            let mut read_buf = BufReader::new(io::stdin().lock());
            let mut write_buf = BufWriter::new(io::stdout().lock());
            let mut buf_to_string = String::new();

            let n = read!(read_buf, buf_to_string).parse::<usize>().unwrap();

            let mut hash = HashMap::with_capacity(n);
            for _ in 0..n {
                let key = read!(read_buf, buf_to_string);
                *hash.entry(key).or_insert(0) += 1;
            }

            let mut vec: Vec<(String, i32)> = hash.into_iter().collect();
            vec.sort_unstable_by_key(|(_, v)| *v);

            let max = vec.last().unwrap_or(&("none".to_string(), 0)).1;
            let mut most_popular = Vec::new();
            while let Some(last) = vec.pop() {
                if last.1 == max {
                    most_popular.push(last.0);
                } else {
                    break
                }
            }

            most_popular.sort_unstable();

            if let Some(book) = most_popular.first() {
                write!(write_buf, "{}", book)?;
            }

            Ok(())
        }
    }
}

fn main() -> io::Result<()> {
    ok!(())
}