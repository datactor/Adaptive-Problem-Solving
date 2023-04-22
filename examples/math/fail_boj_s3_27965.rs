use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn bin_exp(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut exp = exponent;
    let mut base_modulus = base % modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base_modulus) % modulus;
        }
        base_modulus = (base_modulus * base_modulus) % modulus;
        exp /= 2;
    }

    result
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input)?;
    let input_data: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let op_modulus = input_data[1];
    let mut ml_modulus = 0;
    let mut square = 0;

    for number in (1..=input_data[0]).rev() {
        let mut tmp = number;
        let mut len = 0;
        while tmp > 0 {
            len += 1;
            tmp /= 10;
        }
        let number_modulus = number % op_modulus;
        let square_modulus = bin_exp(10, square, op_modulus);
        ml_modulus += number_modulus * square_modulus % op_modulus;
        ml_modulus %= op_modulus;
        square += len;
    }

    write!(writer, "{}", ml_modulus)?;

    Ok(())
}

// use std::io::{self, BufRead, BufReader, BufWriter, Write};
//
// fn main() -> io::Result<()> {
//     let mut reader = BufReader::new(io::stdin().lock());
//     let mut writer = BufWriter::new(io::stdout().lock());
//     let mut input = String::new();
//     reader.read_line(&mut input)?;
//     let input_data: Vec<u64> = input
//         .split_whitespace()
//         .map(|s| s.parse().unwrap())
//         .collect();
//     let op_modulus = input_data[1];
//     let mut ml_modulus = 0;
//     let mut square = 0;
//     let mut base = 1;
//     let mut prev_digit_sum = 0;
//
//     for number in (1..=input_data[0]).rev() {
//         let mut digit_sum = prev_digit_sum;
//         let mut tmp = number;
//         while tmp > 0 {
//             digit_sum += tmp % 10;
//             tmp /= 10;
//         }
//         let cur_digit_sum = digit_sum;
//         let square_modulus = base % op_modulus;
//         let number_modulus = number % op_modulus;
//         ml_modulus += cur_digit_sum * square_modulus * number_modulus;
//         ml_modulus %= op_modulus;
//         prev_digit_sum = cur_digit_sum;
//         base = (base * 10) % op_modulus;
//     }
//
//     write!(writer, "{}", ml_modulus)?;
//
//     Ok(())
// }