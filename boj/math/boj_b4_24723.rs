// https://www.acmicpc.net/problem/24723

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let n = buffer.trim().parse::<u32>()?;
    print!("{}", 2_i32.pow(n));
    Ok(())
}
