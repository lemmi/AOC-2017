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

        let cs: Vec<char> = trimmed.chars().collect();
        let us: Vec<u32> = cs.iter().filter_map(|x| x.to_digit(19)).collect();
        if cs.len() != us.len() {
            println!("error: only digits are allowed!");
            continue 'main;
        }

        let l = us.len();
        let i1 = us.iter();
        let i2 = us.iter().cycle().skip(l/2);

        let sum = i1.zip(i2).fold(0u32, |sum, (&x1, &x2)| sum + if x1 == x2 { x1 } else { 0u32 });

        println!(">>> sum: {}", sum);
    }
}
