fn main() {
    let mut input = [1,2,0,4,3,0,5,0];
    // let mut input = [1,2,0,0,0,3,6];

    for i in 0..input.len(){
        for x in i..input.len(){
            if input[i] == 0{
                input[i] = input[x];
                input[x] = 0;
            }
        }
    }
    for i in input{
        print!("{} ",i);
    }

}