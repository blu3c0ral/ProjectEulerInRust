use std::collections::HashMap;
use utils;

fn get_factors(number: i64, primes: &Vec<i64>) -> HashMap<i64, i64> {
    let mut factors_table: HashMap<i64, i64> = HashMap::new();
    let mut divs: (i64, i64);
    let mut num = number;
    for prime in primes {
        divs = utils::div_rem(num, *prime);
        while divs.1 == 0 {
            if factors_table.contains_key(&prime) {
                factors_table.insert(*prime, *factors_table.get(&prime).unwrap());
            } else {
                factors_table.insert(*prime, 1);
            }
            num = divs.0;
            divs = utils::div_rem(num, *prime);
        }
    }
    return factors_table;
}

pub fn smallest_multiple(upper_bound: i64) -> i64 {
    let primes = utils::get_primes_to_n(upper_bound);
    let mut primes_table: HashMap<i64, i64> = HashMap::new();
    let mut factors_table: HashMap<i64, i64>;
    let mut mul_var: i64 = 1;

    for prime in &primes {
        primes_table.insert(*prime, 0);
    }
    for i in 2..upper_bound {
        if primes_table.contains_key(&i) {
            if *primes_table.get(&i).unwrap() == 0 {
                primes_table.insert(i, 1);
            }
        } else {
            factors_table = get_factors(i, &primes);
            for prime_entry in factors_table {
                if *primes_table.get(&prime_entry.0).unwrap() < prime_entry.1 {
                    primes_table.insert(prime_entry.0, prime_entry.1);
                }
            }
        }
    }
    for prime_entry in primes_table {
        mul_var *= prime_entry.0.pow(prime_entry.1 as u32);
    }

    return mul_var;
}
