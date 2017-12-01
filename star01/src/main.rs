use std::io;

fn main() {
    'main: loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => (),
            Err(error) => {
                println!("{}", error);
                break;
            },
        }

        let trimmed = line.trim();
        if trimmed.len() == 0 {
            continue 'main;
        }

        let iter = trimmed.chars();
        let first = trimmed.chars().take(1);
        let mut sum = 0u32;
        let mut current = 10u32;

        for c in iter.chain(first).map(|x| x.to_digit(10)) {
            let c =  match c {
                Some(x) => x,
                None => {
                    println!("error: only digits are allowed!");
                    continue 'main;
                },
            };
            if c == current {
                sum += c;
            }
            current = c;
            println!("sum: {}, c: {}", sum, c);
        }

        println!(">>> sum: {}", sum);
    }
}
