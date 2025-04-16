/* Ilana's solution:
def split(word): 
    return [char for char in word]  

 
def compareNums(num):
    lst = []
    for i in range(6):
        newNum = num * (i+ 1)
        lst.append(sorted(split(str(newNum))))
    return (lst[0] == lst[1] == lst[2] == lst[3] == lst[4] == lst[5])

 
i = 1;
while(True):
    if (compareNums(i)):
        print(i)
        break
    i+=1
*/

pub fn solve() {
    println!("The least positive value x, where x is an anagram of 2x, 3x, 4x, 5x, and 6x, is {}", get_least_multiple_fold_anagram(6));
}

fn get_least_multiple_fold_anagram(n:u32) -> u32 {
    let mut i = 1;
    loop {
        if compare_nums(i, n) {
            return i;
        }
        i += 1
    }
}

fn compare_nums(i:u32, n:u32) -> bool {
    let mut original_vec = i.to_string().chars().collect::<Vec<char>>();
    original_vec.sort();
    for mul in 2..n {
        let new_num = i * mul;
        let mut vec = new_num.to_string().chars().collect::<Vec<char>>();
        vec.sort();
        if original_vec != vec {
            return false;
        }
    }
    return true;
}

