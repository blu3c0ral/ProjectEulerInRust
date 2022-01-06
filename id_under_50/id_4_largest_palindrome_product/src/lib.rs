use utils;
/*
I don't think there is a way to get better worst case complexity. So I leave it as is.
*/
fn is_palindrome(number: i64) -> bool {
    let mut num = number;
    let mut reversed_num = 0;

    while num > 0 {
        reversed_num *= 10;
        reversed_num += num % 10;
        num /= 10;
    }

    return reversed_num == number;
}

fn is_triplets_prod(number: i64) -> bool {
    let mut factor_cand = 100;
    let mut divs: (i64, i64);
    while factor_cand <= 999 {
        divs = utils::div_rem(number, factor_cand);
        if divs.1 == 0 {
            if (divs.0 >= 100) & (divs.0 <= 999) {
                return true;
            }
        }
        factor_cand += 1;
    }
    return false;
}

pub fn largest_palindrome() -> i64 {
    let max_prod = 999 * 999;
    let min_prod = 100 * 100;
    let mut candidate = max_prod;
    while candidate >= min_prod {
        if is_palindrome(candidate) {
            if is_triplets_prod(candidate) {
                return candidate;
            }
        }
        candidate -= 1;
    }

    return candidate;
}
