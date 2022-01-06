/*
https://users.rust-lang.org/t/quotient-and-remainder/16093/2
There's a guy here who claims that it could be that the compiler is optimizing this code.
In assembly it comes for free...
*/
pub fn div_rem(x: i64, y: i64) -> (i64, i64) {
    let quot = x / y;
    let rem = x % y;
    (quot, rem)
}

/*
Sieve of Eratosthenes
Code from:
https://medium.com/@alistairisrael/how-fast-is-rust-655f6dd90ff8
*/
fn sieve(primes: &mut Vec<i64>, factor: i64) {
    let mut i = 0;
    while i < primes.len() {
        let value = primes[i];
        if value != factor {
            if value % factor == 0 {
                primes.remove(i);
            }
        }
        i += 1;
    }
}

pub fn get_primes_to_n(n: i64) -> Vec<i64> {
    let mut primes :Vec<i64> = Vec::new();

    for i in 2..=n {
        primes.push(i);
    }
    let mut i = 0;
    while i < primes.len() {
        let factor = primes[i];
        sieve(&mut primes, factor);
        i += 1;
    }

    return primes;
}
