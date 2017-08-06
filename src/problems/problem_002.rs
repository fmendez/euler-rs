fn sum_even_fibonacci_not_exceeding_x(target: u64) -> u64 {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 1;
    let mut c = a + b;
    while c < target {
        sum = sum + c;
        a = b + c;
        b = c + a;
        c = a + b;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::sum_even_fibonacci_not_exceeding_x;

    #[test]
    fn sum_even_fibonacci_not_exceeding_x_test(){
        assert!(sum_even_fibonacci_not_exceeding_x(100) == 44);
        assert!(sum_even_fibonacci_not_exceeding_x(4_000_000) == 4613732);
    }
}

