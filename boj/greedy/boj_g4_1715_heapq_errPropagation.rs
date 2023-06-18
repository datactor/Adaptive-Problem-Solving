// https://www.acmicpc.net/problem/1715
// Error propagate

use std::{
    io::{self, Read, Write, BufRead, BufReader, BufWriter},
    error::Error,
    collections::BinaryHeap,
};

// 클로저(람다 함수) 정의 후 바로 실행하는 패턴; 즉시 실행 함수 표현식.
// Immediately Invoked Function Expression, IIFE: ()()
// Result type을 반환하며 성공시 $type값을 담고, 실패시 에러를 담는다.
macro_rules! read {
    ($reader:expr, $input:expr, $type:ty) => {
        (
            || -> Result<$type, Box<dyn Error>> {
                $input.clear();
                $reader.read_line(&mut $input)?;
                Ok($input.trim().parse::<$type>()?)
            }
        )()  // 빈 괄호 쌍`()`를 붙여서 클로저를 즉시 호출. 이것이 IIFE
    };       // 만약 빈 괄호 쌍을 붙이지 않는다면, 단지 클로저 정의에 불과하며 실행되지 않는다.
}            // 우리가 클로저를 사용할 때, 클로저의 정의에 빈 괄호 쌍을 붙이는 것과 동일하다.

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input:s.split_ascii_whitespace(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> Result<T, Box<dyn Error>>
        where
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.input
            .next()
            .ok_or("Reached end of input")? // Option<T>(Some(T), None으로 반환되는 타입을 Result<T>(Ok(T), Err(e))로 재매핑하는 메서드. 에러처리를 일원화 하기 위해 사용.
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into()) // String타입으로 에러 e를 디버그 문자열로 변환 후, into()메서드를 통해 Box<dyn Error>타입으로 변환하여 타입을 맞춘다.
    }

    fn heapq<T: std::str::FromStr + Ord>(&mut self) -> Result<BinaryHeap<i32>, Box<dyn Error>>
        where
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let n = self.next::<usize>()?;
        let mut heap = BinaryHeap::with_capacity(n);

        // 타입을 T로 받으면 token에 -기호를 붙이기 어렵다.
        while let Ok(token) = self.next::<i32>() {
            // let value = token.parse::<T>().map_err(|e| -> Box<dyn Error> { format!("{:?}", e).into() })?;
            // heap.push(-value);
            heap.push(-token);
        }

        Ok(heap)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut read_buf = BufReader::new(io::stdin().lock());
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();

    // 읽을 때 마다 heapq에 collect
    let n = read!(read_buf, buffer, usize)?;
    let mut hq = (0..n).map(|_| read!(read_buf, buffer, i32).map(|v| -v)).collect::<Result<BinaryHeap<i32>, Box<dyn Error>>>()?;

    // // Scanner 생성
    // io::stdin().lock().read_to_string(&mut buffer)?;
    // let mut scanner = Scanner::new(&buffer);

    // // main entry point에서 수행하여 쉽고 명시적으로 에러를 전파하여 hq생성
    // let n = scanner.next::<usize>()?;
    // let mut hq = BinaryHeap::with_capacity(n);
    //
    // for _ in 0..n {
    //     let deck = -scanner.next::<i32>()?;
    //     hq.push(deck);
    // }

    // // main entry point에서 수행하는 대신, 메서드로 hq생성
    // let mut hq = scanner.heapq::<i32>()?;

    let mut sum = 0;
    let mut tmp = 0;
    while let Some(min) = hq.pop() {
        if tmp == 0 {
            tmp += min;
        } else {
            tmp += min;
            sum += tmp;
            hq.push(tmp);
            tmp = 0;
        }
    }
    write!(write_buf, "{}", -sum)?;

    Ok(())
}