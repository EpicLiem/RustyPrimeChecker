use std::{io::stdin, error::Error};
use std::io;
use std::thread;
use std::sync::mpsc;

// This is the code for the threads
fn Check_Primes(Root_Prime_Check: i128, prime_check: i128, p: i128, t: i128) -> bool {
    println!("Starting Thread: {}-{}",((Root_Prime_Check) / (t)) * (p - 1) + 5, ((Root_Prime_Check + 5) / (t)) * p);
    for i in (((Root_Prime_Check) / (4)) * (p - 1) + 5..((Root_Prime_Check + 5) / (4)) * p).step_by(4) {
        if prime_check % i == 0 || prime_check % (i + 2) == 0 {
            return false;
        }
    }
    return true;
}

fn main() -> Result<(), Box<dyn Error>> {
    //threads
    let t = 6;
    //making threads work
    let t = t + 1;
    let t = t;

    //getting input
    print!("Enter a number to check if it is prime: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let prime_check: i128 = input.trim().parse()?;

    //prepping to start code
    let mut flag = true;
    use std::time::Instant;
    println!("Starting calculation...");
    let now = Instant::now();

    //going through low numbers
    if prime_check <= 1 {
        println!("{} is not a prime number",prime_check);
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(());
    }
    if prime_check == 2 || prime_check == 3 {
        println!("{} is a prime number",prime_check);
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(());
    }
    if prime_check % 2 == 0 || prime_check % 3 == 0 {
        println!("{} is a prime number", prime_check);
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(());
    }

    //prepping to start threads
    let root_prime_check: i128 = (prime_check as f64).sqrt() as i128;
    let root_prime_check = root_prime_check;
    let prime_check = prime_check;
    let (tx, rx) = mpsc::channel();

    //starting threads
    for b in 1..t {
        let p = b;
        let p = p;
        let tx_clone = tx.clone();
        let ctx = tx_clone;
        let t = t;
        thread::spawn(move|| {
            let result = Check_Primes(root_prime_check, prime_check, p, t);
            ctx.send(result).unwrap();
        });
    }
    //getting results from threads
    let mut i = 1;
    for received in rx {
        i += 1;
        if received == false {
            flag = false;
            break;
        }
        if i == 5 {
            break;
        }
        println!("Received: {}", received);
    }
    //printing results
    if flag == true {
        println!("{} is a prime number", prime_check);
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
    if flag == false {
        println!("{} is not a prime number", prime_check);
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
    Ok(())
}
