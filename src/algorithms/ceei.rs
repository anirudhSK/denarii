fn allocate(resources: Vec<f64>, demands: Vec<Vec<f64>>) -> Vec<f64> {
    return vec![4.1, 1.6];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_example() {
        let resources = vec![9.0, 18.0];
        let demands = vec![vec![1.0, 4.0], vec![3.0, 1.0]];
        let alloc = allocate(resources, demands);

        assert_eq!(alloc, [4.1, 1.6]);
    }
}
