use utils;

pub fn largest_prime_factor(upper_bound: i64, max_primes: i64) -> i64 {
    let mut quot = upper_bound;

    let primes = utils::get_primes_to_n(max_primes);
    
    for i in 0..=primes.len() {
        let mut divs = utils::div_rem(quot, primes[i]);
        while divs.1 == 0 {
            quot = divs.0;
            divs = utils::div_rem(quot, primes[i]);
        }
        if primes.binary_search(&quot).is_ok() {
            break;
        }
        
    }

    return quot;
}
