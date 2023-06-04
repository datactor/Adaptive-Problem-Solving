use std::{error::Error, io};

// // O(len(bi) * log(n)) -> Fail
// fn main() -> Result<(), Box<dyn Error>> {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input)?;
//
//     let mut input = input.split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap());
//     let (a, b) = (input.next().unwrap(), input.next().unwrap());
//
//     let x = (a..=b).map(|n| {
//         let bi = format!("{:b}", n);
//         let x = bi.as_bytes();
//         x.iter().map(|&j| j-48).sum::<u8>() as u64
//     }).sum::<u64>();
//     println!("{}", x);
//
//     Ok(())
// }

// O(log n)
fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut input = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap());
    let (a, b) = (input.next().unwrap(), input.next().unwrap());

    println!("{}", check_bin1(b) - check_bin1(a - 1));

    Ok(())
}

fn check_bin1(mut n: u64) -> u64 {
    // 0 ~ 2^idx-1까지의 총 1의 개수(점화식)
    // psum[1] = (0..2^1 - 1).iter() 의 총 1의 개수  = 1     = 2^0 + 2*0
    // psum[2] = (0..2^2 - 1).iter() 의 총 1의 개수  = 4     = 2^1 + 2*1
    // psum[3] = (0..2^3 - 1).iter() 의 총 1의 개수  = 12    = 2^2 + 2*4
    // psum[4] = (0..2^4 - 1).iter() 의 총 1의 개수  = 32    = 2^3 + 2*12
    // psum[5] = (0..2^5 - 1).iter() 의 총 1의 개수  = 80    = 2^4 + 2*32
    let mut psum = [0; 58];
    (1..58).for_each(|i| psum[i] = 2_u64.pow(i as u32 - 1) + 2 * psum[i - 1]);

    let mut cnt = 0;
    let bin_num = format!("{:b}", n)
        .as_bytes()
        .iter()
        .map(|&s| s - 48)
        .collect::<Vec<u8>>();
    let len = bin_num.len();

    for i in 0..len {
        if bin_num[i] == 1 {
            let pow = len - i - 1;
            cnt += psum[pow]; // n보다 작은 수 중 가장 큰 2^i까지의 1의 총 개수를 cnt에 합산
            n -= 2_u64.pow(pow as u32); // 가장 큰 2^i까지 계산했으므로 나머지 값을 추가로 연산해주기 위해 연산할 n값을 바꿔줌
            cnt += n + 1; // (2^(len-i-1)..=n)범위의 1들을 모두 cnt에 합산 하기위해 차례로 cnt에 더해줌
        }
    }
    cnt
}
