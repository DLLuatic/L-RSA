pub mod attacks;

use std::env;
use std::io::{self, Write};
use std::time::{Duration, Instant};

fn main() {
    let mut end = false;
    println!("[main]: Enter loop.");
    while !end {
        print!("[main]: Awaiting command. ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let args: Vec<String> = env::args().collect();
        if args[1] == "autowhb" {
            input = String::from("whb100");
        } else {
            io::stdin().read_line(&mut input).unwrap();
        }

        match input.trim() {
            "whb100" => {
                let mut avgruntime_array = Vec::new();
                for j in 1..100 {
                    let m1: i128 = (j as i128*10).pow(3) / /*First N*/1;
                    let m2: i128 = (j as i128*10).pow(3) / /*Second N*/1;
                    let m3: i128 = (j as i128*10).pow(3) / /*Third N*/1; // The 1s are placeholders
                    let mut all_t: u128 = 0;
                    for i in 0..3 {
                        println!("[main]: Testing WHB.");
                        let clock = Instant::now();
                        let res = attacks::weak_hastad_broadcast::weak_hastad(vec![m1, m2, m3], vec![/*First N*/1, /*Second N*/1, /*Third N*/1]);
                        let elapsed = clock.elapsed();

                        all_t += clock.elapsed().as_micros();

                        println!("[main]: Time elapsed: {:?}", elapsed);
                    }
                    println!("[main]: Average runtime: {}", all_t / 4);
                    avgruntime_array.push(all_t / 100);
                }
                println!("{:?}", avgruntime_array);
                println!("[main]: Ending now...");
                end = true;
            },
            "wiener" => {
                let mut all_t: u128 = 0;
                for i in 0..100 {
                    println!("[main]: Testing Wiener.");
                    let clock = Instant::now();
                    attacks::wieners::wiener(/*d*/1, /*N*/1); // Again, 1s are placeholders
                    let elapsed = clock.elapsed();
 
                    all_t += clock.elapsed().as_micros();
                    println!("[main]: Time elapsed: {:?}", elapsed);
                }
                println!("[main]: Average runtime: {}", all_t / 100);
            },
            _ => {
                println!("[main]: Unrecognised. Leaving.");
                end = true;
            },
        }
    }
}
