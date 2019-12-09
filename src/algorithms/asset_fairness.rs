use super::Algorithm;
use crate::gurobi::ffi::{GurobiOptimizer, GurobiVar};

pub struct AssetFairness {
    prices: Vec<f64>,
}

fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    // Calculate the dot product of two vectors.
    assert_eq!(a.len(), b.len());
    let mut product = 0.0;
    for i in 0..a.len() {
        product += a[i] * b[i];
    }
    product
}

impl Algorithm for AssetFairness {
    fn allocate(&self, resources: &Vec<f64>, demands: &Vec<Vec<f64>>) -> Vec<f64> {
        let num_resources = resources.len();
        assert!(num_resources == self.prices.len());
        for demand in demands {
            assert!(demand.len() == self.prices.len());
        }
        let mut optimizer = GurobiOptimizer::new("mip1");
        let coeffs: Vec<GurobiVar> = demands
            .iter()
            .map(|_| optimizer.add_var('C', true))
            .collect();

        for i in 0..num_resources {
            optimizer.add_constraint(
                &coeffs,
                &demands.iter().map(|demand| demand[i]).collect(),
                '<',
                resources[i],
            );
        }

        for i in 0..demands.len() - 1 {
            optimizer.add_constraint(
                &vec![coeffs[i], coeffs[i + 1]],
                &vec![
                    dot_product(&demands[i], &self.prices),
                    -dot_product(&demands[i + 1], &self.prices),
                ],
                '=',
                0.0,
            )
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
        let prices = vec![2.0, 1.0];
        let alg = AssetFairness { prices };
        let alloc = alg.allocate(&resources, &demands);
        let expected_alloc = [2.52, 2.16];

        assert_eq!(alloc.len(), expected_alloc.len());

        for i in 0..alloc.len() {
            assert!(approx_eq!(f64, alloc[i], expected_alloc[i], epsilon = 0.01));
        }
    }

    #[test]
    fn multiple_reqs() {
        let resources = vec![9.0, 18.0];
        let demands = vec![vec![1.0, 4.0], vec![3.0, 1.0], vec![4.0, 2.0]];
        let prices = vec![2.0, 1.0];
        let alg = AssetFairness { prices };
        let alloc = alg.allocate(&resources, &demands);
        println!("{:?}", alloc);
        let expected_alloc = [1.50, 1.29, 0.90];
        assert_eq!(alloc.len(), expected_alloc.len());
        for i in 0..alloc.len() {
            println!("{}, {}, {}", i, alloc[i], expected_alloc[i]);
            assert!(approx_eq!(f64, alloc[i], expected_alloc[i], epsilon = 0.01));
        }
    }
}
