use number_theory::number_theory;

pub fn problem43() {
    let mut sum = 0_u64;
    let pandigitals = number_theory::permute(vec!(0,1,2,3,4,5,6,7,8,9));
    for pandigital in pandigitals {
        if (100*pandigital[1] + 10*pandigital[2] + pandigital[3]) % 2 != 0 {
            continue;
        }
        if (100*pandigital[2] + 10*pandigital[3] + pandigital[4]) % 3 != 0 {
            continue;
        }
        if (100*pandigital[3] + 10*pandigital[4] + pandigital[5]) % 5 != 0 {
            continue;
        }
        if (100*pandigital[4] + 10*pandigital[5] + pandigital[6]) % 7 != 0 {
            continue;
        }
        if (100*pandigital[5] + 10*pandigital[6] + pandigital[7]) % 11 != 0 {
            continue;
        }
        if (100*pandigital[6] + 10*pandigital[7] + pandigital[8]) % 13 != 0 {
            continue;
        }
        if (100*pandigital[7] + 10*pandigital[8] + pandigital[9]) % 17 != 0 {
            continue;
        }
        sum += pandigital.iter().fold(0, |a,b| return a*10+b);
    }
    println!("the sum of all the special 0-9 pandigitals is {}", sum);
}