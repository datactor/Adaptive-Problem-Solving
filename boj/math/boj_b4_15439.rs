// https://www.acmicpc.net/problem/15439

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let n = buffer.trim().parse::<usize>()?;
    print!("{}", n * (n - 1));
    Ok(())
}
