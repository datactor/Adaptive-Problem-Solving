// https://www.acmicpc.net/problem/17413

use std::collections::VecDeque;

fn print(s: &mut VecDeque<char>) {
    while !s.is_empty() {
        print!("{}", s.pop_back().unwrap());
    }
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let str = buffer.trim_end();

    let mut tag = false;
    let mut s: VecDeque<char> = VecDeque::new();

    for ch in str.chars() {
        if ch == '<' {
            print(&mut s);
            tag = true;
            print!("{}", ch);
        } else if ch == '>' {
            tag = false;
            print!("{}", ch);
        } else if tag {
            print!("{}", ch);
        } else {
            if ch == ' ' {
                print(&mut s);
                print!("{}", ch);
            } else {
                s.push_back(ch);
            }
        }
    }
    print(&mut s);
}
