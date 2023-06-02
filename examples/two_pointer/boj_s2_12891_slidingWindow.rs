use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

macro_rules! get_cases {
    ($reader:expr, $writer:expr, $type:ty) => {
        {
            let mut line = || $reader.next().expect("Failed to get next line").expect("Failed to read line");
            let parts = line().split_ascii_whitespace().map(|s| s.parse::<$type>().expect(&format!("Failed to parse '{}' into '{}'", s, stringify!($type)))).collect::<Vec<$type>>();
            let (s, p) = (parts[0], parts[1]);
            let dna_seq = line().as_bytes().to_vec();
            let target = line().split_ascii_whitespace().map(|s| s.parse::<$type>().expect(&format!("Failed to parse '{}' into '{}'", s, stringify!($type)))).collect::<Vec<$type>>();

            let mut table = [0; 26];

            for i in 0..p {
                table[(dna_seq[i] - b'A') as usize] += 1;
            }

            let mut password_cases = 0;
            if table[0] >= target[0] && table[(b'C' - b'A') as usize] >= target[1] && table[(b'G' - b'A') as usize] >= target[2] && table[(b'T' - b'A') as usize] >= target[3] {
                password_cases += 1;
            }

            for i in 0..s-p {
                table[(dna_seq[i] - b'A') as usize] -= 1;
                table[(dna_seq[i+p] - b'A') as usize] += 1;
                if table[0] >= target[0]
                    && table[(b'C' - b'A') as usize] >= target[1]
                    && table[(b'G' - b'A') as usize] >= target[2]
                    && table[(b'T' - b'A') as usize] >= target[3] {
                    password_cases += 1;
                }
            }

            writeln!($writer, "{}", password_cases)?
        }
    }
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock()).lines();
    let mut write_buf = BufWriter::new(io::stdout().lock());

    get_cases!(read_buf, write_buf, usize);

    Ok(())
}