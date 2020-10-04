// for small values of n only
fn fib2(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        fib2(n - 2) + fib2(n - 1)
    }
}

fn fib4(n: usize, acc: usize) -> usize {
    if n < 2 {
        n
    } else {
        fib2(n - 2) + fib2(n - 1)
    }
}

fn fib5(n: usize) -> usize {
    if n == 0 {
        return n;
    }

    let mut last = 0;
    let mut next = 1;
    for _ in 1..n {
        let tmp = last;
        last = next;
        next = last + tmp;
    }
    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, fib2(4));
        assert_eq!(3, fib4(4, 0));
        assert_eq!(3, fib5(4));
        assert_eq!(12586269025, fib5(50));
    }
}
