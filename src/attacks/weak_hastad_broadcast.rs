/* Implementation of a weak variant of the hastad broadcast attack */

// Input of shape ([m1, m2, ...], [n1, n2, ...])
pub fn weak_hastad(message_vector: Vec<i128>, n_vector: Vec<i128>) -> f64 {
    let (mut x, factor) = crt_constructive_proof(&message_vector, &n_vector);
    let n_prod: i128 = n_vector.iter().product();
    while x < 0 {
        x += factor;
        if n_prod < x {
            panic!("[WHB]: Message size OVERFLOW");
        }
    }
    while n_prod < x {
        x -= factor;
        if x < 0 {
            panic!("[WHB]: Message size UNDERFLOW");
        }
    }
    f64::powf(x as f64, 1.0 / 3.0)
}

// This will output a solution to the system of congruences (x), alongside a
// factor (a), by which any multiple of that factor will also be a solution to
// the system of congruences, in the shape (x, a)
fn crt_constructive_proof(a_vec: &Vec<i128>, n_vec: &Vec<i128>) -> (i128, i128) {
    let mut comb_n = n_vec[0];
    let mut x_fin: i128 = a_vec[0]; // failsave value
    for i in 1..n_vec.len() {
        let (gcd, x, y): (i128, i128, i128);
        if comb_n > n_vec[i] {
            (gcd, x, y) = extended_euclidean_algorithm(comb_n, n_vec[i]);
        } else {
            (gcd, y, x) = extended_euclidean_algorithm(n_vec[i], comb_n);
        }
        if gcd != 1 {
            println!("[WHB]: Ns NOT COPRIME, CONSIDER factoring");
        }
        x_fin = (a_vec[i] * x * comb_n) + (x_fin * y * n_vec[i]);
        comb_n *= n_vec[i];
    }
    (x_fin, comb_n)
}

// Returns (gcd, x, y (Bezout identity factors))
fn extended_euclidean_algorithm(a: i128, b: i128) -> (i128, i128, i128) {
    let mut rs = vec![a, b];
    let mut svec = vec![1, 0];
    let mut tvec = vec![0, 1];
    let mut q: i128;
    while rs[rs.len() - 1] != 0 {
        q = rs[rs.len() - 2] / rs[rs.len() - 1];
        rs.push(rs[rs.len() - 2] - (q * rs[rs.len() - 1]));
        svec.push(svec[svec.len() - 2] - (q * svec[svec.len() - 1]));
        tvec.push(tvec[tvec.len() - 2] - (q * tvec[tvec.len() - 1]));
    }
    (rs[rs.len() - 2], svec[svec.len() - 2], tvec[tvec.len() - 2])
}
