use std::{
    io::{self, BufRead},
    fmt::{Write, Error},
};

fn main() -> Result<(), Error> {
    let lines = io::stdin().lock().lines();
    let mut writer = String::new();

    for reader in lines {
        let mut res = "yes";
        let line = reader.unwrap();
        if line != "0" {
            let bytes = line.as_bytes();
            let len = bytes.len();
            for i in 0..len {
                if bytes[i] != bytes[len - 1 - i] {
                    res = "no";
                    break
                }
            }
            writeln!(writer, "{}", res)?;
        }
    }
    print!("{}", writer);

    Ok(())
}