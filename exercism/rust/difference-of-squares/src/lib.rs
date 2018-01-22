pub fn sum(n: usize) -> usize {
    n * (n+1) / 2
}

pub fn square_of_sum(n: usize) -> usize {
    sum(n).pow(2)
}

pub fn sum_of_squares(n: usize) -> usize {
    let mut sum = 0;
    for i in 1..n+1 {
        sum += i*i;
    }
    sum
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
