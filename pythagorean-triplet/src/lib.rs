use std::collections::HashSet;
use std::iter;

/************************
 * QUESTION FOR MENTORS:
 *
 * I am not really satisfied with the time complexity of this solution,
 * which runs in O(n^2) where n is the size of the sum of triplets.
 *
 * I investigated more efficient solutions based on the method for constructing
 * triplets from 2 integers m and n.
 *
 * See:
 *  - https://www.mathsisfun.com/numbers/pythagorean-triples.html
 *  - https://www.geeksforgeeks.org/generate-pythagorean-triplets/,
 *
 * However, these solutions failed the test cases for sums 90, 840, and 30000, for which
 * m and n have non-integer values. As such, I could not figure out a way to enumerate
 * values of m and n that would exhaustively produce triplets from the test cases.
 *
 * (I also had questions as to how to analyze the time complexity,
 *  and find a suitable upper bound for m in this strategy.)
 *
 * If you have any hints on how to solve the problem more efficiently, or could
 * set my mind at ease that a more efficient solution is not possible, I would appreciate it.
 *
 * :)
 *
 ************************/

/// Given the sum `sum`,
/// - return all possible Pythagorean triplets, which produce the said sum
/// - or an empty HashSet if there are no such triplets.
/// - Note that you are expected to return triplets in [a, b, c] order, where a < b < c
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..sum)
        .flat_map(|b| {
            (1..b)
                .zip(iter::repeat(b))
                .filter_map(|(a, b)| find_triplet(a as f64, b as f64, sum as f64))
        })
        .collect()
}

fn find_triplet(a: f64, b: f64, sum: f64) -> Option<[u32; 3]> {
    let c = ((a * a) + (b * b)).sqrt();
    if a + b + c == sum {
        Some([a as u32, b as u32, c as u32])
    } else {
        None
    }
}

/*
  // imperative version
  pub fn find(sum: u32) -> HashSet<[u32]> {
    let mut triplets = HashSet::<[u32; 3]>::new();
    for b in 1..sum {
        for a in 1..b {
            if let Some(triplet) = find_triplet(a as f64, b as f64, sum as f64) {
                triplets.insert(triplet);
            }
        }
    }
    triplets
}
 */
