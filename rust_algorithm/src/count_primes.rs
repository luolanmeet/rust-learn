// 计算素数
fn count_primes(n: i32) -> i32 {
    if n <= 2 {
        return 0;
    }
    let mut primes = vec![true; n as usize];
    primes[0] = false;
    primes[1] = false;
    let mut count = 0;
    for i in 2..n {
        if primes[i as usize] {
            count += 1;
            let mut j = i * i;
            while j < n {
                primes[j as usize] = false;
                j += i;
            }
        }
    }
    return count;
}

#[test]
fn test_one() {
    assert_eq!(count_primes(25), 9);
}
