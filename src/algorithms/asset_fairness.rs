use crate::gurobi::ffi::GurobiOptimizer;
use float_cmp::approx_eq;

fn allocate(resources: Vec<f64>, demands: Vec<Vec<f64>>) -> Vec<f64> {
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
    // TODO: Express these 6.0 and -7.0 using resources and demands vectors.
    optimizer.add_constraint(&vec![x, y], &vec![6.0, -7.0], '=', 0.0);
    optimizer.optimize("max");

    return vec![
        *optimizer.solutions.get(&x).unwrap(),
        *optimizer.solutions.get(&y).unwrap(),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_example() {
        let resources = vec![9.0, 18.0];
        let demands = vec![vec![1.0, 4.0], vec![3.0, 1.0]];
        let alloc = allocate(resources, demands);

        let expected_alloc = [2.5, 2.2];

        assert_eq!(alloc.len(), expected_alloc.len());

        for i in 0..alloc.len() {
            approx_eq!(f64, alloc[i], expected_alloc[i]);
        }
    }
}
