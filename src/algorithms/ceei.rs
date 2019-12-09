use super::Algorithm;
use crate::gurobi::ffi::{GurobiOptimizer, GurobiVar};

pub struct Ceei {}

impl Algorithm for Ceei {
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
        let alg = Ceei {};
        let alloc = alg.allocate(&resources, &demands);

        let expected_alloc = [4.09, 1.63];

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
        let resources = vec![9.0, 18.0, 15.0];
        let demands = vec![
            vec![1.0, 4.0, 2.0],
            vec![3.0, 1.0, 3.0],
            vec![4.0, 2.0, 1.0],
        ];
        let alg = Ceei {};
        let alloc = alg.allocate(&resources, &demands);
        let expected_alloc = [4.09, 1.63, 0.0];
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
