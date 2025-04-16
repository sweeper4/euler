use number_theory::number_theory;
//this is facstoer beuilging up from smaller primes instead of decomponsing all potatntial primes but imma not write that
pub fn solve() {
    let mut sum = 0;
    let primes = number_theory::prime_sieve(750000);
    for i in primes.iter() {
        if *i > 9 {
            let mut j = *i/10;
            let mut a = true;
            while j > 0 {
                if !primes.contains(&j) {
                    a = false;
                    break;
                }
                j /= 10;
            }
            j = *i;
            let mut k = 1;
            while 10_u64.pow(k) < j {
                if !primes.contains(&(j % 10_u64.pow(k))) {
                    a = false;
                    break;
                }
                k+=1;
            }
            if a {
                sum += i;
            }
        }
    }
    println!("lotsas things sum to {} I did a drunk code", sum);
}