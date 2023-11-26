/* Simple implementation of the Wiener's attack on the RSA cryptsystem.

This only works when d < (N^(1/4))/3. Given this condition is true, k/d is
among the continued fractions expansion convergents of e/N.

To actually get d, we
1) Turn e/N into its continued fraction form.
2) Find our convergents of the continued fraction (as k/d).
3) Check if, given some d, we can produce a factorisation of N.
4) If it does, return d. This constitutes a total break.

If we don't end up with a valid d before d > (N^(1/4))/3, then d is too large
for wiener's attack.
*/

// Central function. This shall derive d from e (pub_exp) and N (comp_num).
pub fn wiener(pub_exp: i32, comp_num: i32) -> i32 {
    let mut continued_fra_form = Vec::new();
    let cff = as_continued_fraction(&comp_num, &pub_exp, continued_fra_form);
    let (ks, ds) = find_convergents(cff);
    // At this point, d might be among ds.
    for i in 2..ds.len() {
        if ks[i] == 0 {
            continue;
        } else if ds[i] as f32 > ((comp_num as f32).sqrt().sqrt() / 3.0) {
            panic!("Wiener's attack failed. Private exponent too large!");
        }
        let phi_n = ((&pub_exp * &ds[i]) - 1) / &ks[i];
        if check(phi_n as f32, &(comp_num as f32)) {
            return ds[i];
        }
    }
    panic!("Wiener's attack failed. Not a single d found!");
}

// The "as_continued_fraction" function is recursive, so we define its result
// outside the function (to not constantly initialize a new vector).
fn as_continued_fraction(denom: &i32, numer: &i32, mut init_vector: Vec<i32>) -> Vec<i32> {
    let i: i32 = numer / denom;
    let q: i32 = numer - (denom * &i);
    init_vector.push(i);
    if q == 0 {
        return init_vector;
    } else {
        let final_vector = as_continued_fraction(&q, &denom, init_vector);
        return final_vector;
    }
}

fn find_convergents(cf: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut i_ks = vec![0, 1]; // Initial ks
    let mut i_ds = vec![1, 0]; // Initial ds
    for i in &cf {
        i_ks.push(i * &i_ks[i_ks.len() - 1] + &i_ks[i_ks.len() - 2]);
        i_ds.push(i * &i_ds[i_ds.len() - 1] + &i_ds[i_ds.len() - 2]);
    }
    (i_ks, i_ds)
}

// By solving the equation
// x^(2) - ((n - phi(n)) + 1)x + n = 0
// we must produce a factorisation, otherwise d is not valid.
fn check(phi_n: f32, n: &f32) -> bool {
    let p_h = ((n - phi_n) + 1.0) / 2.0;
    let factor_one = p_h + (p_h.powf(2.0) - n).sqrt();
    let factor_two = p_h - (p_h.powf(2.0) - n).sqrt();
    factor_one * factor_two - n < 0.0001 // Remember: we're working with
                                         // floating point numbers here
}

// TODO: Add some Test documentation here
#[cfg(test)]
mod tests {
    use crate::attacks::wieners::{as_continued_fraction, find_convergents, check};
    #[test]
    fn cf_returns() {
        let mut continued_fra_form = Vec::new();
        let denom = 200;
        let numer = 649;
        assert_eq!(as_continued_fraction(&denom, &numer, continued_fra_form), vec![3, 4, 12, 4]);
    }

    #[test]
    fn fc_returns() {
        assert_eq!(find_convergents(vec![0, 1, 5, 2, 2]),(vec![0, 1, 0, 1, 5, 11, 27], vec![1, 0, 1, 1, 6, 13, 32]));
    }

    #[test]
    fn check_returns() {
        assert_eq!(check(89964 as f32, &(90581 as f32)), true)
    }
}
