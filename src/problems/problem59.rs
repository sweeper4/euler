use std::{fs::File, io::{BufReader, BufRead}};
use regex::Regex;

pub fn solve() {
    let file = File::open("src/assets/problem59_cipher.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut count = 0;
    for line in lines {
        let line = line.unwrap();
        let regex = Regex::new(r"(\d+)").unwrap();
        for a in 'a' as u8..('z' as u8 +1) {
            for b in 'a' as u8..('z' as u8 +1) {
                for c in 'a' as u8..('z' as u8 +1) {
                    let key = vec![a,b,c];
                    let mut decrypted = vec![];
                    let mut errored = false;
                    for (_, [digit]) in regex.captures_iter(&line).map(|x| x.extract()) {
                        let encrypted_char = digit.parse::<u32>().unwrap();
                        let dec = encrypted_char as u8 ^ key[decrypted.len()%3];
                        if (dec < 32 && dec > 0) || dec == 127 {
                            errored = true;
                            break;
                        }
                        decrypted.push(dec as char);
                    }
                    let mut found_the = false;
                    let mut found_for = false;
                    for i in 0..decrypted.len() {
                        if !found_the && i+2 < decrypted.len() && (decrypted[i] == 'T' || decrypted[i] == 't') && decrypted[i+1] == 'h' && decrypted[i+2] == 'e' {
                            found_the = true;
                        }
                        if !found_for && i+2 < decrypted.len() && (decrypted[i] == 'F' || decrypted[i] == 'f') && decrypted[i+1] == 'o' && decrypted[i+2] == 'r' {
                            found_for = true;
                        }
                    }
                    if errored || !found_the || !found_for {
                        continue;
                    }
                    let sum: u64 = decrypted.clone().iter().map(|x| *x as u8 as u64).sum();
                    let decrypted :String = decrypted.iter().collect();
                    println!("{:?}, {}: {:?}", key, sum, decrypted);
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn test() {
        solve()
    }
}