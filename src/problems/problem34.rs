/* Ilana's solution:
i = 3
facts = [1,1,2,6,24,120,720,5040,40320,362880]
while (True):
    string = str(i)
    sum = 0
    for digit in string:
        sum += facts[int(digit)]
    if (sum == i) :
        print(sum)
    i+=1
*/

pub fn problem34() {
    println!("The sum of all numbers that are equal to the sum of the factorial of their digits is {}",sum_values_whose_value_is_the_sum_of_their_digits_factorial());
}

fn sum_values_whose_value_is_the_sum_of_their_digits_factorial() -> usize {
    let mut i = 3;
    let facts = [1,1,2,6,24,120,720,5040,40320,362880];
    let mut final_sum = 0;
    let mut iterations_since_last_equality = 0;
    while iterations_since_last_equality < 1000000 {
        let mut temp_i = i;
        let mut sum = 0;
        while temp_i > 0 {
            sum += facts[temp_i % 10];
            temp_i /= 10;
        }
        if i == sum {
            final_sum += i;
            iterations_since_last_equality = 0;
        }
        iterations_since_last_equality += 1;
        i += 1;
    }
    return final_sum;
}