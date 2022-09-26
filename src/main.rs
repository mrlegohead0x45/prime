use std::collections::HashSet;
struct Prime {
    primes: HashSet<i32>,
}

impl Prime {
    fn new() -> Self {
        Self {
            primes: HashSet::new(),
        }
    }

    fn add_prime(&mut self, prime: i32) {
        // println!("found new {}", prime);
        self.primes.insert(prime);
    }

    fn is_prime(&mut self, n: i32) -> bool {
        if self.primes.contains(&n) {
            return true;
        }
        if n == 2 || n == 3 {
            self.add_prime(n);
            return true;
        }
        if n <= 1 || n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        let stop = f32::sqrt(n as f32) as i32;
        while i <= stop {
            // println!("testing {i}");
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        self.add_prime(n);
        return true;
    }

    fn nth_prime(&mut self, n: i32) -> i32 {
        let mut count: i32 = 0;
        let mut i: i32 = 0;
        loop {
            i += 1;
            if self.is_prime(i) {
                count += 1;
                // println!("found {}th", count);
            }
            if count == n {
                return i;
            }
        }
    }
}

fn main() {
    // // println!("{}", nth_prime(10_000));
    let mut p = Prime::new();
    // // let mut i = 0;
    // // while i < 50 {
    // //     println!("{}", p.nth_prime(i));
    // //     i += 1;
    // // }
    let args: Vec<String> = std::env::args().collect();
    let n = args[1].parse::<i32>().unwrap();
    let _x = p.nth_prime(n);
}
