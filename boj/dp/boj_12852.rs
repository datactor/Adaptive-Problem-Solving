// https://www.acmicpc.net/problem/12852

use std::{
    collections::HashMap,
    error::Error,
    io::{self, prelude::*, BufWriter},
};

struct MinSteps {
    previous_steps: HashMap<i32, (i32, Vec<i32>)>,
}

impl MinSteps {
    fn new() -> MinSteps {
        MinSteps {
            previous_steps: HashMap::from([
                (1, (0, vec![1])),
                (2, (1, vec![1, 2])),
                (3, (1, vec![1, 3])),
            ]),
        }
    }

    fn get_steps_to_one(&mut self, num: i32) -> (i32, Vec<i32>) {
        if let Some(prev_steps) = self.previous_steps.get(&num) {
            return (*prev_steps).to_owned();
        }

        let mut possible_steps = Vec::new();
        let (mut step_count, mut steps_sequence);

        if num % 3 == 0 {
            (step_count, steps_sequence) = self.get_steps_to_one(num / 3);
            possible_steps.push((step_count + 1, steps_sequence));
        }
        if num % 2 == 0 {
            (step_count, steps_sequence) = self.get_steps_to_one(num / 2);
            possible_steps.push((step_count + 1, steps_sequence));
        }
        if (num - 1) % 3 == 0 || (num - 1) % 2 == 0 {
            (step_count, steps_sequence) = self.get_steps_to_one(num - 1);
            possible_steps.push((step_count + 1, steps_sequence));
        }
        if (num - 2) % 3 == 0 {
            (step_count, steps_sequence) = self.get_steps_to_one(num - 2);
            steps_sequence.push(num - 1);
            possible_steps.push((step_count + 2, steps_sequence));
        }

        let (step_count, mut steps_sequence) = possible_steps.iter().min().unwrap().to_owned();

        steps_sequence.push(num);
        self.previous_steps
            .insert(num, (step_count, steps_sequence.to_owned()));

        (step_count, steps_sequence)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_line(&mut input)?;

    let num: i32 = input.trim().parse()?;
    let mut min_steps = MinSteps::new();

    let (step_count, steps_sequence) = min_steps.get_steps_to_one(num);

    writeln!(output, "{}", step_count)?;
    for step in steps_sequence.iter().rev() {
        write!(output, "{} ", step)?;
    }

    Ok(())
}
