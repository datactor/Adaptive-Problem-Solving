use std::{
    io::{self, BufRead, BufReader},
    collections::HashMap,
    cell::RefCell,
    rc::Rc,
    fmt::Write
};

macro_rules! read {
    ($reader:expr, $input:expr) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            $input.split_ascii_whitespace()
        }
    }
}

macro_rules! ok {
    (()) => {
        {
            let mut read_buf = BufReader::new(io::stdin().lock());
            let mut buf_to_string = String::new();

            let n = read!(read_buf, buf_to_string)
                .next()
                .expect("Failed to get next iter")
                .parse::<usize>()
                .expect("Failed to parse");

            let mut root = Node::default();

            for _ in 0..n {
                let rooms = read!(read_buf, buf_to_string)
                    .skip(1)
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<String>>();

                root.insert(&rooms, 0);
            }

            let writer = Rc::new(RefCell::new(String::new()));
            root.write(0, &writer)?;

            print!("{}", writer.borrow());

            Ok(())
        }
    }
}

#[derive(Default)]
struct Node {
    child: HashMap<String, Node>,
}

impl Node {
    fn insert(&mut self, v: &[String], idx: usize) {
        if idx == v.len() {
            return;
        }

        // if !self.child.contains_key(&v[idx]) {
        //     self.child.insert(v[idx].clone(), Node::default());
        // }
        // self.child.get_mut(&v[idx]).unwrap().insert(v, idx + 1);

        // 아래의 메서드 체인은 위의 코드의 조합과 같다.
        self.child.entry(v[idx].clone()).or_default().insert(v, idx + 1);
    }

    fn write(&self, depth: usize, output: &Rc<RefCell<String>>) -> io::Result<()> {
        let mut sorted_keys = self.child.keys().cloned().collect::<Vec<String>>();
        sorted_keys.sort();
        for parent in sorted_keys {
            if let Some(child) = self.child.get(&parent) {
                for _ in 0..depth {
                    write!(output.borrow_mut(), "{}", "--").expect("Failed to write");
                }
                writeln!(output.borrow_mut(), "{}", parent).expect("Failed to writeln");
                child.write(depth + 1, output)?;
            }
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    ok!(())
}