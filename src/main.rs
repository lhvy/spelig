mod keys;

use arrayvec::ArrayVec;
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

        let mut events: Vec<_> = trimmed_line
            .chars()
            .flat_map(|c| {
                let mut events: ArrayVec<Event, 3> = ArrayVec::new();

                if c.is_ascii_uppercase() {
                    events.push(Event::ShiftDown);
                    events.push(Event::KeyPress(c.to_ascii_lowercase()));
                    events.push(Event::ShiftUp);
                } else {
                    events.push(Event::KeyPress(c));
                }

                events
            })
            .collect();

        for i in 0..events.len() - 1 {
            let random: f32 = rng.gen();

            match events[i] {
                Event::ShiftUp | Event::ShiftDown if random > 0.85 => events.swap(i, i + 1),
                Event::ShiftUp | Event::ShiftDown => {}

                _ if random > 0.975 => events.swap(i, i + 1),
                _ => {}
            }

            if random > 0.98 {
                if let Event::KeyPress(ref mut c) = events[i] {
                    if let Some(chars) = keys::KEYS.get(c) {
                        *c = *chars.choose(&mut rng).unwrap();
                    }
                }
            }
        }

        let mut chars = Vec::new();
        let mut is_uppercase = false;
        for event in events {
            match event {
                Event::ShiftUp => is_uppercase = false,
                Event::ShiftDown => is_uppercase = true,
                Event::KeyPress(c) => chars.push(if is_uppercase {
                    c.to_ascii_uppercase()
                } else {
                    c
                }),
            }
        }

        println!("{}", chars.into_iter().collect::<String>());

        line.clear();
    }

    Ok(())
}

#[derive(Debug)]
enum Event {
    ShiftUp,
    ShiftDown,
    KeyPress(char),
}
