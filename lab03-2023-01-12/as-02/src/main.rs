fn main() {
    let input:usize  = 100;
    let answer:usize = find_sum(input);
    println!("1 + 2 + 3 + ... + 100  =  {}",answer);

}
fn find_sum(x:usize) ->usize {
    let mut sum:usize =0;
    for i in 1..x+1{
        sum += i;
    }
    return sum;
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_find_sum() {
        assert_eq!(find_sum(100), 5050);
   }

}
