mod keys;

use rand::prelude::SliceRandom;
use rand::Rng;
use std::io::{self, stdin, BufRead};

fn main() -> io::Result<()> {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();
    let mut rng = rand::thread_rng();

    loop {
        stdin.read_line(&mut line)?;
        let eof = line.is_empty();
        if eof {
            break;
        }

        let trimmed_line = line.trim_end(); // Remove final newline
        if trimmed_line.is_empty() {
            continue;
        }
        let mut chars: Vec<_> = trimmed_line.chars().collect();
        for i in 0..chars.len() - 1 {
            if rng.gen_bool(0.025) {
                chars.swap(i, i + 1);
            }

            if rng.gen_bool(0.02) {
                if let Some(c) = keys::KEYS.get(&chars[i]) {
                    chars[i] = *c.choose(&mut rng).unwrap();
                }
            }
        }

        println!("{}", chars.into_iter().collect::<String>());

        line.clear();
    }

    Ok(())
}
