use std::cmp::max;
use pyo3::prelude::*;
use statrs::distribution::{Binomial,Discrete};

mod optimise;
mod distributions;

// Constants
pub const D_MAX: usize = 50;

// calculates the expectation of a random variable.
// Supports the following distributions:
// 'P' - Poisson
// 'B' - Binomial
// 'N' - Negative Binomial
#[pyfunction]
#[pyo3(signature = (x, param_one, param_two, st_out, second_param_one=None, second_param_two=None, distribution=None, r=None, p=None))]
fn expectation(x: usize, param_one: f64, param_two: f64, st_out: f64, second_param_one: Option<f64>, second_param_two: Option<f64>,distribution: Option<char>, r: Option<f64>, p: Option<f64>) -> PyResult<(f64,f64)> {
    // Generate required parameters and pmf's
    let dfw_p: f64 = p.unwrap_or(0.8);


    // Pregenerate the binomial distribution
    let mut binom_pmf = [[0.0; D_MAX+1]; D_MAX+1];
    for i in 0..D_MAX+1 {
        let binom_distr = Binomial::new(dfw_p, i as u64).unwrap();
        for j in 0..D_MAX+1 {
            binom_pmf[i][j] = (binom_distr.pmf(j as u64)) as f64;
        }
    }
 

    let dist: char = distribution.unwrap_or('P');
    let d1_pmf: [f64; D_MAX] = distributions::generate_distribution::distribution(dist, param_one, second_param_one);
    let d2_pmf: [f64; D_MAX] = distributions::generate_distribution::distribution(dist, param_two, second_param_two);

    // Calculate the order quantity 
    let q: f64 = f64::min(f64::max(st_out-x as f64,0.0), r.unwrap_or(st_out));

    // Calculate the expectation
    let mut exp: f64 = 0.0;
    let mut exp_first_stage: f64 = 0.0;

    for (d1_val,d1_pmf_i) in d1_pmf.iter().enumerate() {
        
        // First stage shortage
        let shortage_p1: usize = max(d1_val as isize -x as isize,0) as usize;
        for j in 0..shortage_p1+1  {
            let fs = d1_pmf_i * binom_pmf[shortage_p1][j] * (f64::max(d1_val as f64 - x as f64,0.0)- j as f64);
            exp_first_stage += fs;
            exp += fs;
        }

        // Second stage shortage
        for (d2_val, d2_pmf_i) in d2_pmf.iter().enumerate() {
            let shortage_p2: usize = max(d2_val as isize - max(x as isize-d1_val as isize,0)-q as isize,0) as usize;
            for j in 0..shortage_p2+1 {
                exp += d1_pmf_i * d2_pmf_i * binom_pmf[shortage_p2][j] * (f64::max(d2_val as f64-f64::max(x as f64 -d1_val as f64,0.0)-q,0.0)-j as f64) ;
            }
        }
    }
    Ok((exp, exp_first_stage))
}

// Implements a lookahead policy which also optimises an order quantity
#[pyfunction]
#[pyo3(signature = (x, param_one, param_two,  max_q, second_param_one=None, second_param_two=None, distribution=None, p=None,cu=None,co=None,cdfw=None))]
fn lookahead(x: usize, param_one: f64, param_two: f64, max_q: usize,second_param_one: Option<f64>, second_param_two: Option<f64>,distribution: Option<char>, p: Option<f64>, cu: Option<f64>, co: Option<f64>, cdfw: Option<f64>) -> PyResult<(f64,f64,usize)> {
    // Costs
    let cost_params = (cu.unwrap_or(18.0), co.unwrap_or(1.0), cdfw.unwrap_or(0.0));
    // Generate required parameters and pmf's
    let dfw_p: f64 = p.unwrap_or(0.8);


    // Pregenerate the binomial distribution
    let mut binom_pmf = [[0.0; D_MAX+1]; D_MAX+1];
    for i in 0..D_MAX+1 {
        let binom_distr = Binomial::new(dfw_p, i as u64).unwrap();
        for j in 0..D_MAX+1 {
            binom_pmf[i][j] = (binom_distr.pmf(j as u64)) as f64;
        }
    }



    let dist: char = distribution.unwrap_or('P');
    let d1_pmf: [f64; D_MAX] = distributions::generate_distribution::distribution(dist, param_one, second_param_one);
    let d2_pmf: [f64; D_MAX] = distributions::generate_distribution::distribution(dist, param_two, second_param_two);

    // Calculate the expectation
    let mut exp: f64 = 0.0;
    let mut exp_first_stage: f64 = 0.0;
    

    // Find the optimal q
    let q: f64 = optimise::min_func::minimise_q(max_q, x, d1_pmf, d2_pmf, binom_pmf, cost_params);
   
    // First stage shortage
    for (d1_val,d1_pmf_i) in d1_pmf.iter().enumerate() {
        
        // First stage shortage
        let shortage_p1: usize = max(d1_val as isize -x as isize,0) as usize;
        for j in 0..shortage_p1+1  {
            let fs = d1_pmf_i * binom_pmf[shortage_p1][j] * (f64::max(d1_val as f64 - x as f64,0.0)- j as f64);
            exp_first_stage += fs;
            exp += fs;
        }

        // Second stage shortage
        for (d2_val, d2_pmf_i) in d2_pmf.iter().enumerate() {
            let shortage_p2: usize = max(d2_val as isize - max(x as isize-d1_val as isize,0)-q as isize,0) as usize;
            for j in 0..shortage_p2+1 {
                exp += d1_pmf_i * d2_pmf_i * binom_pmf[shortage_p2][j] * (f64::max(d2_val as f64-f64::max(x as f64 -d1_val as f64,0.0)-q,0.0)-j as f64) ;
            }
        }
    }

    Ok((exp_first_stage, exp, q as usize))
}

/// A Python module implemented in Rust.
#[pymodule]
fn wrapper_alternative(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(expectation, m)?)?;
    m.add_function(wrap_pyfunction!(lookahead, m)?)?;
    Ok(())
}
