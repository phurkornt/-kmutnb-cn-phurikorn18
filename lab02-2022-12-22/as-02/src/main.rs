fn main() {
    let input  = 5;
    let mut add = 1;
    for i in 0..input{
            for x in 0..input-i-1{
                print!(" ");
            }
            for x in 0..add{
                print!("*");
            }
            add += 2;
            println!();
    }
}