fn sum_of_divs(n_elements: i64, n: i64) -> i64 {
    return n * n_elements * (n_elements + 1) / 2;
}

/*
https://users.rust-lang.org/t/quotient-and-remainder/16093/2
There's a guy here who claims that it could be that the compiler is optimizing this code.i64
In assembly it comes for free...
*/
fn div_rem(x: i64, y: i64) -> (i64, i64) {
    let quot = x / y;
    let rem = x % y;
    (quot, rem)
}

fn get_n_elements(upper_bound: i64, n: i64) -> i64 {
    let divs = div_rem(upper_bound, n);
    let n_elements = divs.0;
    if divs.1 == 0 {
        return n_elements - 1;
    } else {
        return n_elements;
    }
}

pub fn multiples_of_3_or_5(upper_bound: i64) -> i64 {
    return sum_of_divs(get_n_elements(upper_bound, 3), 3) + sum_of_divs(get_n_elements(upper_bound, 5), 5) - sum_of_divs(get_n_elements(upper_bound, 15), 15)
}
