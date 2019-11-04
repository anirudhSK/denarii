/* DRF pseudo-code
 * R = <r_1, r_2, ..., r_m> total resource capacities
 * C = <c_1, c_2, ..., c_m> consumed resources, initially 0
 * s_i (i = 1..n) user i's dominant shares, initially 0
 * U_i = <u_i,1, ... u_i, m> (i = 1..n) resources given to user i, initially 0
 *
 * pick user i with lowest dominant share s_i
 * D_i <- demand of user_i's next task
 * if C + D_i <= R then
 *   C = C + D_i        update consumed vector
 *   U_i = U_i + D_i    update i's allocation vector
 *   s_i = max_{j=1..m} {u_{i, j} / r_j}
 * else
 *   return             the cluster is full
 * end if
 */

/* Given 9 CPUs, 18 GB RAM and two users, where A runs tasks with demand vector
 * <1 CPU, 4 GB> and user B runs tasks with demand vector <3 CPUs, 1 GB> each.
 * The DRF allocation is then given by the solution to the following
 * optimization problem:
 *
 * max (x, y)
 * subject to
 *      x + 3y <= 9     (CPU constraint)
 *      4x + y <= 18    (Memory constraint)
 *      2x/9 = y/ 3     (Equalize dominant shares)
 *
 *
 * Given R = <r_1, r_2, ..., r_m> total resource capabilities
 * Demand vectors, where i = 1..n
 *      d_i = <d_{i, 1}, d_{i, 2}, ..., d_{i, m}>
 *      s_i = max_{j=1..m} {}
 *
 * Resource j
 * sum_{i = 1..n} d_{i, j} <= r_j
 *
 *
 */

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
    optimizer.add_constraint(&vec![x, y], &vec![2.0 / 9.0, -1.0 / 3.0], '=', 0.0);
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

        let expected_alloc = [3.0, 2.0];

        assert_eq!(alloc.len(), expected_alloc.len());

        for i in 0..alloc.len() {
            approx_eq!(f64, alloc[i], expected_alloc[i]);
        }
    }
}
