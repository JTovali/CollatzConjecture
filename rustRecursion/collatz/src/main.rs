use std::env;
use std::process;

// compute the collatz sequence length
fn collatz_length(n: u64) -> u64 {
    if n == 1 {  // base case
        0
    } else if n % 2 == 0 {
        1 + collatz_length(n / 2)
    } else {
        1 + collatz_length(3 * n + 1)                                                            
    }
}

// find the 10 smallest integers with the longest collatz sequence length
fn find_the_top_10_collatz (start: u64, end: u64) -> Vec<(u64, u64)> {
    // only hold at most 10 pairs
    let mut top: Vec<(u64, u64)> = Vec::new();

    for n in start..=end {
        let len = collatz_length(n);

        // of length is the same, keep the smaller integer
        if let Some(pos) = top.iter().position(|&(_, l)| l == len) {
            if n < top[pos].0 {
                top[pos].0 = n;
            }
            continue;
        }

        // if new length, and still room to add, then add length
        if top.len() < 10 {
            top.push((n, len));
        } else {
            // new length but top is full, then replace with the worst case
            let mut worst_idx = 0;
            for i in 1..top.len() {
                let (ni, li) = top[i];
                let (nw, lw) = top[worst_idx];
                if li < lw || (li == lw && ni > nw) {
                    worst_idx = i;                             
                }
            }                                    
            let (worst_n, worst_l) = top[worst_idx];

            // replace if worst if the pair is better
            if len > worst_l || (len == worst_l && n < worst_n) {
                top[worst_idx] = (n, len);
            }
        }
    }
    // sort - length descending and interger ascending
    top.sort_by(|a, b| {
        b.1.cmp(&a.1)   // length desc
            .then_with(|| a.0.cmp(&b.0)) // int asc
    });
    top
}

fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
}
            
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: filename start end");
        process::exit(1);
    }

    // parse args
    let x = args[1].parse::<u64>().unwrap_or_else(|_| {
        println!("Error: Start value must be an integer.");
        process::exit(1);
    });

    let y = args[2].parse::<u64>().unwrap_or_else(|_| {
        println!("Error: End value must be an integer.");
        process::exit(1);
    });

    // create the min and max range to not create an error
    let start = x.min(y);
    let end = x.max(y);

    let mut top_10 = find_the_top_10_collatz(start, end);

    clear_screen();

    println!("Sorted based on sequence length");
    for (n, len) in &top_10 {
        println!("{:>15}{:>15}", n, len);
    }

    println!();
    println!("Sorted based on integer size");
    top_10.sort_by(|a, b| b.0.cmp(&a.0));
    for (n, len) in &top_10 {
        println!("{:>15}{:>15}", n, len);
    }
}
