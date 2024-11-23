// Find the smallest q in a one dimensional search. Does a full search of all possible q values and returns the smallest q value and the corresponding expectation. 

pub fn minimise_q(max_q: usize, x: usize, demand_p1: [f64; crate::D_MAX], demand_p2: [f64; crate::D_MAX], dfw_p2: [[f64; crate::D_MAX+1]; crate::D_MAX+1], cost_params: (f64, f64, f64)) -> f64 {
    // Create a list of acceptable order quantites
    let mut acceptable_q: Vec<usize> = Vec::with_capacity(max_q + 1);
    acceptable_q.extend(0..=max_q);
    

    let (cu,co,cdfw) = cost_params;

    let mut best_q = (0, 0.0);

    for q in acceptable_q.iter() {
        // Calculate second stage expectation balancing shortage and holding costs
        let mut exp: f64 = 0.0;
        for (d1_val,d1_pmf_i) in demand_p1.iter().enumerate() {
            // Second stage expectation

            // On hand moving into the second stage
            let x_2 = f64::max(x as f64 - d1_val as f64, 0.0)+*q as f64;


            for (d2_val, d2_pmf_i) in demand_p2.iter().enumerate() {
                let shortage_p2: usize = std::cmp::max(d2_val as isize - x_2 as isize,0) as usize;
                for j in 0..shortage_p2+1 {
                    exp += d1_pmf_i * d2_pmf_i * dfw_p2[shortage_p2][j] * (cdfw * j as f64 + cu * (shortage_p2 - j) as f64);
                }
                exp += d1_pmf_i * d2_pmf_i * co * f64::max(x_2-d2_val as f64,0.0);

            }
        }
        // Check if the expectation is better than the current best
        if q == &0 {
            best_q.0 = *q;
            best_q.1 = exp;
        } else {
            if exp < best_q.1 {
                best_q.0 = *q;
                best_q.1 = exp;
            }
        }
    }

    return best_q.0 as f64;
}