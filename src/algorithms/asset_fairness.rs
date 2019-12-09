use super::Algorithm;
use crate::gurobi::ffi::GurobiOptimizer;
use float_cmp::approx_eq;

pub struct AssetFairness {}

impl Algorithm for AssetFairness {
    fn allocate(&self, resources: &Vec<f64>, demands: &Vec<Vec<f64>>) -> Vec<f64> {
        // TODO: Current handles only 2 resources and 2 demands; need to generalize
        assert!(demands[0].len() == 2);
        assert!(demands[1].len() == 2);
        assert!(resources.len() == 2);

        let mut optimizer = GurobiOptimizer::new("mip1");
        let x = optimizer.add_var('C', true);
        let y = optimizer.add_var('C', true);
        optimizer.add_constraint(
            &vec![x, y],
            &vec![demands[0][0], demands[1][0]],
            '<',
            resources[0],
        );
        optimizer.add_constraint(
            &vec![x, y],
            &vec![demands[0][1], demands[1][1]],
            '<',
            resources[1],
        );
        optimizer.add_constraint(&vec![x, y], &vec![6.0, -7.0], '=', 0.0);
        optimizer.optimize("max");

        return vec![
            *optimizer.solutions.get(&x).unwrap(),
            *optimizer.solutions.get(&y).unwrap(),
        ];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_example() {
        let resources = vec![9.0, 18.0];
        let demands = vec![vec![1.0, 4.0], vec![3.0, 1.0]];
        let alg = AssetFairness {};
        let alloc = alg.allocate(&resources, &demands);

        let expected_alloc = [2.5, 2.2];

        assert_eq!(alloc.len(), expected_alloc.len());

        for i in 0..alloc.len() {
            approx_eq!(f64, alloc[i], expected_alloc[i]);
        }
    }
}
