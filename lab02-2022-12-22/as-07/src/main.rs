use std::io;
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed");
    let input:usize = input
    .trim()
    .parse()
    .expect("Failed input number");

    let mut fibo = [ 0 , 1];
    let mut Xn = 0;
    if input >= fibo.len(){
        for i in 1..input{
            Xn = fibo[0] + fibo[1];
            fibo[0] = fibo[1];
            fibo[1] = Xn;
        }
    }else{
        Xn = fibo[input];
    }
    print!("Output : {}",Xn)
}
