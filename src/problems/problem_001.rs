fn sum_multiples_of_a_and_b_below_z(a: u64, b: u64, z: u64) -> u64 {
    divible_by(a, z) + divible_by(b, z) - divible_by(a*b, z)
}

fn divible_by(n: u64, z: u64) -> u64 {
    let q = (z-1) / n;
    (n * (q * (q+1) ) ) / 2
}

#[cfg(test)]
mod test {
    use super::sum_multiples_of_a_and_b_below_z;

    #[test]
    fn sum_multiples_of_a_and_b_below_z_test() {
        assert!(sum_multiples_of_a_and_b_below_z(3, 5, 10) == 23);
        assert!(sum_multiples_of_a_and_b_below_z(3, 5, 1000) == 233168);
        assert!(sum_multiples_of_a_and_b_below_z(3, 5, 1000000000) == 233333333166666668);
    }

}
