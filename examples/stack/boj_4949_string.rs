// https://www.acmicpc.net/problem/4949

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();

    for line in lines {
        if line == ".".to_string() {
            break
        }

        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '[' => stack.push(c),
                '(' => stack.push(c),
                ']' =>
                    if stack.len() != 0 && stack[stack.len()-1] == '[' {
                        stack.pop();
                    } else {
                        stack.push(']');
                        break; // stack에 넣었던 left괄호를 지울 수 없으면 stack을 채워두고 yes 리턴
                    },
                ')' =>
                    if stack.len() != 0 && stack[stack.len()-1] == '(' {
                        stack.pop();
                    } else {
                        stack.push(')');
                        break; // stack에 넣었던 left괄호를 지울 수 없으면 stack을 채워두고 yes 리턴
                    },
                    _ => {},
            }
        }
        writeln!(output, "{}", match stack.is_empty() {
            true => "yes",
            false => "no",
        })?;
    }
    Ok(())
}