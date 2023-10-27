fn gcd(a: u32, b: u32) -> u32 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn is_prime(n: u32) -> bool {
    for i in 2..n {
        let gcd_res = gcd(n, i);
        if gcd_res != 1 {
            return false;
        }
    }
    true
}

fn sieve_rec<'a>(list: Vec<u32>, sieve_nums: Vec<u32>) -> Vec<u32> {
    if sieve_nums.is_empty() {
        return list;
    }
    if list.is_empty() {
        return list;
    }
    let sieve_by = sieve_nums.get(0).unwrap().clone();
    let new_list: Vec<u32> = list
        .into_iter()
        .filter(|x| *x == sieve_by || *x % sieve_by != 0)
        .collect();
    let new_sieve_nums: Vec<u32> = sieve_nums
        .into_iter()
        .filter(|x| *x != sieve_by && *x % sieve_by != 0)
        .collect();
    sieve_rec(new_list, new_sieve_nums)
}

fn sieve(n: u32) -> Vec<u32> {
    let until = f64::sqrt(n as f64) as u32;
    let list: Vec<u32> = (2..=n).collect();
    let sieve_nums: Vec<u32> = (2..=until).collect();
    // let mut res = sieve_rec(list, sieve_nums);

    let res = sieve_rec(list, sieve_nums);
    // res.reverse();

    res
}

pub fn nth(n: u32) -> u32 {
    // todo!("What is the 0-indexed {n}th prime number?")
    let mut until = 1000;
    loop {
        let primes_until_1000 = sieve(until);
        let prime = primes_until_1000.into_iter().nth(n as usize);
        match prime {
            Some(v) => return v,
            None => until = until * 10,
        }
    }
}
