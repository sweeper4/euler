fn sum_evens_less_than( max: usize) -> usize {
    let mut fb_generator = Fb::new();
    let mut i = 0;
    let mut total = 0;
    while fb_generator.get(i) < max {
        if fb_generator.get(i) % 2 == 0 {
            total = total + fb_generator.get(i)
        }
        i = i + 1
    }
    return total
}

struct Fb {
    cache: Vec<usize>
}

impl Fb {
    fn get( &mut self,index: usize) -> usize {
        match self.cache.get(index) {
            Some( a ) => {
                return *a 
            },
            None => {
                let r = self.get(index - 2) + self.get( index - 1 );
                self.cache.push(r);
                return r
            }
        }
    }

    fn new() -> Fb {
        let mut value = Fb {
            cache: Vec::new()
        };
        value.cache.push(0);
        value.cache.push(1);
        return value;
    }
}
pub fn solve() {
    println!("total of even fibonacci numbers = {}", sum_evens_less_than(4000000));
}