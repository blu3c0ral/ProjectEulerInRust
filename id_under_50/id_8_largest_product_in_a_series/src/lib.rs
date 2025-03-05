use rayon::prelude::*;

pub fn largest_product_in_a_series_sliding(series: &str, num_digits: usize) -> i64 {
    let digits: Vec<i64> = series
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as i64))
        .collect();

    if digits.len() < num_digits {
        return 0;
    }

    // Calculate initial product
    let mut product: i64 = digits[0..num_digits].iter().product();
    let mut max_product: i64 = product;

    // Slide the window
    for i in 0..digits.len() - num_digits {
        // If the digit we're removing wasn't zero, divide by it
        if digits[i] != 0 {
            product = product / digits[i];
        } else {
            // Previous product was 0, recalculate entirely
            product = digits[i + 1..i + num_digits + 1].iter().product();
            max_product = max_product.max(product);
            continue;
        }

        // If the digit we're adding isn't zero, multiply by it
        if digits[i + num_digits] != 0 {
            product = product * digits[i + num_digits];
        } else {
            // New product will be 0
            product = 0;
        }

        max_product = max_product.max(product);
    }

    max_product
}

pub fn largest_product_in_a_series_bf(series: &str, num_digits: usize) -> i64 {
    let mut max_product = 0;
    let mut series_chars = series.chars();
    let mut series_vec = Vec::new();

    while let Some(c) = series_chars.next() {
        series_vec.push(c.to_digit(10).unwrap() as i64);
    }

    for i in 0..series_vec.len() - num_digits {
        let mut product = 1;
        for j in 0..num_digits {
            product *= series_vec[i + j];
        }
        if product > max_product {
            max_product = product;
        }
    }

    max_product
}

pub fn largest_product_in_a_series_mt(series: &str, num_digits: usize) -> i64 {
    // Convert string to digits just once
    let digits: Vec<i64> = series
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as i64))
        .collect();

    // If there are insufficient digits, return early
    if digits.len() < num_digits {
        return 0;
    }

    // Calculate max product in parallel
    (0..=digits.len() - num_digits)
        .into_par_iter()
        .map(|i| {
            // Early return 0 if any digit in window is 0
            if digits[i..i + num_digits].contains(&0) {
                return 0;
            }

            // Otherwise compute product
            digits[i..i + num_digits].iter().product()
        })
        .max()
        .unwrap_or(0)
}
