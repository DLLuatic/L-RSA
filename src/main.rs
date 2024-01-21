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
            "whb100" => {
                let mut all_t: u128 = 0;
                for i in 0..99 {
                    println!("[main]: Testing WHB.");
                    let clock = Instant::now();
                    let res = attacks::weak_hastad_broadcast::weak_hastad(vec![2, 3, 2], vec![3, 5, 7]);
                    let elapsed = clock.elapsed();

                    all_t += clock.elapsed().as_micros();

                    println!("[main]: Time elapsed: {:?}", elapsed);
                }
                println!("[main]: Average runtime: {}", all_t / 100);
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
