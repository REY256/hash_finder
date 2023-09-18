use clap::Parser;
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::{ops::Range, sync::Mutex, time::Instant};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'N')]
    n: usize,
    #[arg(short = 'F')]
    f: usize,
}

const STEP: u128 = 1_000_000;

fn main() {
    let args = Args::parse();

    let n = args.n;
    let f = args.f;

    let mut range: Range<u128> = 0..STEP;

    let lines_count: Mutex<usize> = Mutex::new(0);

    let start = Instant::now();

    loop {
        if *lines_count.lock().unwrap() >= f {
            break;
        }

        let new_range = range.clone();
        new_range
            .collect::<Vec<u128>>()
            .par_iter()
            .map(|i| {
                let hash = num_to_hash(*i);

                let res = zero_count(&hash);

                if res == n {
                    println!("{i}, {hash}");
                    *lines_count.lock().unwrap() += 1;
                }
            })
            .count();

        let start = range.end;
        let end = range.end + STEP;
        range = start..end;
    }

    println!("\n{:?}", start.elapsed());
}

fn num_to_hash(num: u128) -> String {
    let mut hasher = Sha256::new();
    hasher.update(num.to_string());
    let result = hasher.finalize();
    format!("{result:x}")
}

fn zero_count(hash: &str) -> usize {
    let mut zero_counter = 0;

    let char_vec: Vec<char> = hash.chars().collect();

    for i in (0..char_vec.len()).rev() {
        let e = char_vec[i];
        if e == '0' {
            zero_counter += 1;
        } else {
            break;
        }
    }

    zero_counter
}
