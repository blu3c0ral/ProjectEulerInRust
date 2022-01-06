use utils;

fn sum_of_divs(n_elements: i64, n: i64) -> i64 {
    return n * n_elements * (n_elements + 1) / 2;
}

fn get_n_elements(upper_bound: i64, n: i64) -> i64 {
    let divs = utils::div_rem(upper_bound, n);
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
