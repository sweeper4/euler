
//:)
fn some_function_called_here(upperLimit: u128) -> u128{
    let mut x=1;
    let mut y=0;
    let mut count=0;
    while x <= upperLimit {
        x = x+y;
        y = x-y;
        if x%2 ==0{
            count+=x;
        }
    }
    return count;
            
    
}
pub fn problem2(){
    println!("bananas{}", some_function_called_here(400000000000000000000))
} 
