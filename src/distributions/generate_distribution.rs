
// Allow a range of distributions to be generated
// Distributions allowed are based on "Fitting Discrete Distributions on the First Two Moments" Adan (1995)
// Will generate the pmf
use statrs::distribution::{Poisson,Binomial, NegativeBinomial,Discrete};

pub fn distribution(dist_type: char, param_1: f64, param_2: Option<f64>) -> [f64; crate::D_MAX] {

    if dist_type == 'P' {
        // Poisson

        // Error check if the demand is too large
        if param_1 > 28.0 {
            panic!("Demand param has to be less than 28, currently {}. To fix increase D_MAX", param_1);
        }
        let mut param_1 = param_1;
        if param_1 == 0.0 {
            param_1 = 0.0001;
        }

        let poisson_distr = Poisson::new(param_1).unwrap();
        let pmf: [f64; crate::D_MAX] = core::array::from_fn(|i| poisson_distr.pmf(i as u64));
        return pmf;

    } else if dist_type == 'B' {
        // Error check if the demand is too large
        if param_1 > 50.0 {
            panic!("Demand param has to be less than 50, currently {}. To fix increase D_MAX", param_1);
        }
        // Binomial
        let param_2 = param_2.expect("You need to provide a second parameter for the binomial distribution");
        let binom_distr = Binomial::new(param_2, param_1 as u64).unwrap();
        let pmf: [f64; crate::D_MAX] = core::array::from_fn(|i| binom_distr.pmf(i as u64));

        return pmf;

    } else if dist_type == 'N' {
        // Error check if the demand is too large
        // Note, theres no checking yet for the negative binomial distribution
        // todo: Check if this is the correct value

        // Negative Binomial    
        let param_2 = param_2.expect("You need to provide a second parameter for the negative binomial distribution");
        let neg_binom_distr = NegativeBinomial::new(param_1, param_2).unwrap();
        let pmf: [f64; crate::D_MAX] = core::array::from_fn(|i| neg_binom_distr.pmf(i as u64));
        return pmf;
    } else {
        panic!("Distribution type not recognised");
    }
}
 