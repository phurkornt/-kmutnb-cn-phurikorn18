fn main() {
    let input = 5;
    for i in 0..input{
        for x in 0..input{
            if i == x {
                print!("x ");
            }else if x == 0 || x == input-1{
                print!("x ");
            }else{
                print!("O ");
            }
        }
        println!();
    }
}