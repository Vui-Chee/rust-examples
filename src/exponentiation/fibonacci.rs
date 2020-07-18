/// Computes the nth fibonacci number by
/// repeating [f(n-1), f(n)] * [[0, 1], [1, 1]].
pub fn fibonacci(n: usize) -> usize {
    let mut state: [usize; 2] = [0, 1];

    if n <= 1 {
        return state[n];
    }

    for _i in 1..n {
        state = [0 * state[0] + 1 * state[1], 1 * state[0] + 1 * state[1]]
    }

    state[1]
}

#[cfg(test)]
mod tests {
    use super::fibonacci;

    #[quickcheck]
    fn test_fib(n: usize) -> bool {
        if n >= 80 {
            return true;
        }

        let fib_n = fibonacci(n);

        if n >= 2 {
            let fib_n_1 = fibonacci(n - 1);
            let fib_n_2 = fibonacci(n - 2);
            return fib_n == fib_n_1 + fib_n_2;
        }

        fib_n == n
    }
}
