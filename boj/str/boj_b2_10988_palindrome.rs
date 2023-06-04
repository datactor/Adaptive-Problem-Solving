use std::{
    io::{self, BufRead},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let s = input.trim().as_bytes();
    let len = s.len();
    let mut ans = 1;

    for i in 0..len/2 {
        let a = s[i];
        let b = s[len-i-1];

        if a != b {
            ans = 0;
            break
        }
    };
    println!("{ans}");
}