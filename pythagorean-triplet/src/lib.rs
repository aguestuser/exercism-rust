use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;

/// Given the sum `sum`,return
/// - all possible Pythagorean triplets, which produce the said sum
/// - an empty HashSet if there are no such triplets.
///
/// we factor the pythagorean theorum from a^2 + b^2 = c^2
/// to isolate b and c in terms of a and "sum", where "sum" = a + b + c:
///
/// c = sum - a - b
/// b = ((b + c)^2 - a^2) / 2(b + c)
///   = ((sum -a)^2) - a^2 / 2(sum - a + c)
///
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1_u32..(sum / 3_u32))
        .into_par_iter()
        .filter_map(|a| {
            let (b, remainder) = calculate_b_and_remainder(a, sum - a);
            let c = sum - a - b;
            match remainder == 0 && a < b {
                true => Some([a, b, c]),
                false => None,
            }
        })
        .collect::<HashSet<[u32; 3]>>()
}

fn calculate_b_and_remainder(a: u32, b_plus_c: u32) -> (u32, u32) {
    let (numerator, denominator) = ((b_plus_c).pow(2) - a.pow(2), 2 * b_plus_c);
    (numerator / denominator, numerator % denominator)
}
