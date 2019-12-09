use super::Algorithm;
use crate::gurobi::ffi::{GurobiOptimizer, GurobiVar};

pub struct Drf {}

impl Algorithm for Drf {
    fn allocate(&self, resources: &Vec<f64>, demands: &Vec<Vec<f64>>) -> Vec<f64> {
        let num_resources = resources.len();
        for demand in demands {
            assert!(demand.len() == num_resources);
        }
        let mut optimizer = GurobiOptimizer::new("mip1");
        let coeffs: Vec<GurobiVar> = demands
            .iter()
            .map(|_| optimizer.add_var('C', true))
            .collect();

        // Add constraint for each type of resources.
        for i in 0..num_resources {
            optimizer.add_constraint(
                &coeffs,
                &demands.iter().map(|demand| demand[i]).collect(),
                '<',
                resources[i],
            );
        }

        // Compute dominant shares for each demand.
        let mut dominant_shares: Vec<f64> = Vec::new();
        for demand in demands {
            let mut max = -0. / 0.;
            for j in 0..demand.len() {
                let share = demand[j] / resources[j];
                max = f64::max(max, share);
            }

            dominant_shares.push(max);
        }

        // Equalize dominant shares.
        for i in 0..demands.len() - 1 {
            optimizer.add_constraint(
                &vec![coeffs[i], coeffs[i + 1]],
                &vec![dominant_shares[i], -dominant_shares[i + 1]],
                '=',
                0.0,
            );
        }
        optimizer.optimize("max");

        return coeffs
            .iter()
            .map(|var| *optimizer.solutions.get(&var).unwrap())
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn simple_example() {
        let resources = vec![9.0, 18.0];
        let demands = vec![vec![1.0, 4.0], vec![3.0, 1.0]];
        let alg = Drf {};
        let alloc = alg.allocate(&resources, &demands);

        let expected_alloc = [3.0, 2.0];

        assert_eq!(alloc.len(), expected_alloc.len());

        for i in 0..alloc.len() {
            assert!(
                approx_eq!(f64, alloc[i], expected_alloc[i], epsilon = 0.01),
                "{} != {}",
                alloc[i],
                expected_alloc[i]
            );
        }
    }

    #[test]
    fn multiple_reqs() {
        let resources = vec![9.0, 18.0];
        let demands = vec![vec![1.0, 4.0], vec![3.0, 1.0], vec![4.0, 2.0]];
        let alg = Drf {};
        let alloc = alg.allocate(&resources, &demands);
        let expected_alloc = [1.79, 1.20, 0.90];
        assert_eq!(alloc.len(), expected_alloc.len());
        for i in 0..alloc.len() {
            assert!(
                approx_eq!(f64, alloc[i], expected_alloc[i], epsilon = 0.01),
                "{} != {}",
                alloc[i],
                expected_alloc[i]
            );
        }
    }
}
