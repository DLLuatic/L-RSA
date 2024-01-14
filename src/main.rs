pub mod attacks;

use std::io::{self, Write};
use std::time::{Duration, Instant};

fn main() {
    let mut end = false;
    println!("[main]: Enter loop.");
    while !end {
        print!("[main]: Awaiting command. ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "whb" => {
                println!("[main]: Testing WHB.");
                let clock = Instant::now();
                attacks::weak_hastad_broadcast::weak_hastad(vec![0, 3, 4], vec![3, 4, 5]);
                let elapsed = clock.elapsed();

                println!("[main]: Time elapsed: {:?}", elapsed);
            },
            "wiener" => {
                println!("[main]: Testing Wiener.");
                let clock = Instant::now();
                attacks::wieners::wiener(17993, 90581);
                let elapsed = clock.elapsed();

                println!("[main]: Time elapsed: {:?}", elapsed);
            },
            _ => {
                println!("[main]: Unrecognised. Leaving.");
                end = true;
            },
        }
    }
}
