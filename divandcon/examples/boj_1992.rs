use std::{
    io::{self, prelude::*, BufWriter, StdoutLock},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();

    let v: Vec<&[u8]> = (0..n).map(|_| lines.next().unwrap().as_bytes()).collect();

    div_conq(0, 0, n, &v, &mut output);

    Ok(())
}

fn div_conq(x: usize, y: usize, n: usize, v: &Vec<&[u8]>, output: &mut BufWriter<StdoutLock>) {
    let mut tmp = 0;
    for i in x..x+n {
        for j in y..y+n {
            if v[i][j] == 49 {
                tmp += 1
            }
        }
    }

    if tmp == 0 {
        write!(output, "0");
        return
    } else if tmp == n.pow(2) {
        write!(output, "1");
        return
    } else {
        write!(output, "(");
        div_conq(x, y, n/2, v, output);
        div_conq(x, y+n/2, n/2, v, output);
        div_conq(x+n/2, y, n/2, v, output);
        div_conq(x+n/2, y+n/2, n/2, v, output);
        write!(output, ")");
        return
    }
}