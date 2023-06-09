macro_rules! solve (
    () => {
        {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            print!("{}", input.trim().len())
        }
    }
);

fn main() {
  solve!()
}