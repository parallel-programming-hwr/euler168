use num_bigint::{BigUint, ToBigUint};
use num_integer::Integer;
use num_traits::ToPrimitive;
use std::sync::mpsc::*;
use std::ops::MulAssign;
use std::thread;
use std::time::Instant;

fn main() {
    let num_threads: u64 = num_cpus::get() as u64;
    let (tx, rx) = channel::<BigUint>();
    let (sen_timing, rec_timing) = channel::<bool>();
    let end: BigUint = ubig_pow(10.to_biguint().unwrap(), 100);
    println!("Starting {} threads...", num_threads);
    for i in 0u64..num_threads {
        let tx = tx.clone();
        let sen_timing = sen_timing.clone();
        let end = end.clone();
        thread::spawn(move || {
            get_rotatable(tx, sen_timing, 10+i, end, num_threads);
        });
        println!("Started thread {}", i);
    }
    // spawns a new thread to calculate the iterations / second
    thread::spawn(move || {
        let mut last_sent = Instant::now();
        let mut iterations = 0;
        loop {
            rec_timing.recv().unwrap();
            iterations += 1;
            if last_sent.elapsed().as_secs() > 10 {
                println!("{:.2} iter/sec", iterations as f64/last_sent.elapsed().as_secs() as f64);
                last_sent = Instant::now();
                iterations = 0;
            }
        }
    });
    loop {
        println!("{}", rx.recv().unwrap());
    }
}

/// searches for numbers according to [euler-168](https://projecteuler.net/problem=168)
/// the resulting numbers are sent via the tx-sender into a channel.
/// every iteration a timing-boolean is sent in via the sen_time-sender to calculate
/// the speed of the iterations
fn get_rotatable(tx: Sender<BigUint>, sen_time: Sender<bool>, start: u64, end: BigUint, step: u64) {
    let mut num: BigUint = start.to_biguint().unwrap();
    let zero: BigUint = 0.to_biguint().unwrap();
    while num < end {
        let mut digits = ubig_digits(num.clone());
        digits.rotate_right(1);
        let num_rotated = ubig_from_digits(digits);
        if (num_rotated % &num) == zero {
            let _ = tx.send(num.clone());
        }
        num += step;
        let _ = sen_time.send(true);
    }
}

/// returns a vector containing the digits of the BigUint
fn ubig_digits(big_number: BigUint) -> Vec<u32> {
    let mut num: BigUint = big_number;
    let mut digits: Vec<u32> = vec![];
    let zero: BigUint = 0.to_biguint().unwrap();
    let ten: BigUint = 10.to_biguint().unwrap();
    while num > zero {
        let (quot, rem) = num.div_rem(&ten);
        num = quot;
        digits.push(rem.to_u32().unwrap());
    }
    digits
}

/// returns a BigUint for a vector of digits
fn ubig_from_digits(digits: Vec<u32>) -> BigUint {
    let mut num: BigUint = 0.to_biguint().unwrap();
    for (index, digit) in digits.iter().enumerate() {
        num += digit * (10 as u32).pow(index as u32);
    }
    num
}

/// returns ubig^exp
fn ubig_pow(ubig: BigUint, exp: usize) -> BigUint {
    let mut num = ubig.clone();
    for _ in 0..exp {
        num.mul_assign(ubig.clone())
    }
    num
}