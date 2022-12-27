fn main() {
    let input  = 10;
        for i in 0..input{
            for x in 0..i{
                print!(" ");
            }
            let cal = ( input - i ) + ( input - i - 1 );
            for x in 0..cal{
                print!("*");
            }

            println!();
        }
    }