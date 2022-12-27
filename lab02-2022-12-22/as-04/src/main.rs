fn main() {

    let input = 14;
    let mut cal = input;
    let mut isCal = true;
    print!("{} = ",cal);
    while(true){
      let mut show = 0;
      if cal % 2 == 0{
          cal = cal / 2;
          show = 2;
      }else if cal % 3 == 0{
          cal = cal / 3;
          show = 3;
      }else{
          if(isCal == true){
              print!("1 * {}",cal);
          }else{
              print!("* {}",cal);
          }
          break;
      }
      if show != 0 && cal % 2 == 0 ||  cal % 3 == 0 {
          print!("{} * ",show);
      }else{
          print!("{} ",show);
      }
      isCal = false;
    }
}