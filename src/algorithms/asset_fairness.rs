use crate::gurobi::ffi::GurobiOptimizer;

fn allocate(resources: Vec<f64>, demands: Vec<Vec<f64>>) -> Vec<f64> {
    let mut optimizer = GurobiOptimizer::new("mip1");
    let x = optimizer.add_var('C', true);
    let y = optimizer.add_var('C', true);
    optimizer.add_constraint(&vec![x, y], &vec![demands[0][0], demands[1][0]], '<', resources[0]);
    optimizer.add_constraint(&vec![x, y], &vec![demands[0][1], demands[1][1]], '<', resources[1]);
    optimizer.add_constraint(&vec![x, y], &vec![6.0, -7.0], '=', 0.0);
    optimizer.optimize("max");

    return vec![*optimizer.solutions.get(&x).unwrap(), *optimizer.solutions.get(&y).unwrap()];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_example() {
        let resources = vec![9.0, 18.0];
        let demands = vec![vec![1.0, 4.0], vec![3.0, 1.0]];
        let alloc = allocate(resources, demands);

        assert_eq!(alloc, [2.52, 2.16]);
    }
}
