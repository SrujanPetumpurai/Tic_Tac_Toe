use std::io::{self, Read};

fn check_winner(tic_tac:&Vec<Vec<u32>>)->Option<u32>{
    for col in 0..3 {
        if tic_tac[0][col] != 0 && tic_tac[0][col] == tic_tac[1][col] && tic_tac[1][col] == tic_tac[2][col] {
            return Some(tic_tac[0][col]);
        }
    }

    // Check rows
    for row in 0..3 {
        if tic_tac[row][0] != 0 && tic_tac[row][0] == tic_tac[row][1] && tic_tac[row][1] == tic_tac[row][2] {
            return Some(tic_tac[row][0]);
        }
    }

    // Check diagonals
    if tic_tac[0][0] != 0 && tic_tac[0][0] == tic_tac[1][1] && tic_tac[1][1] == tic_tac[2][2] {
        return Some(tic_tac[0][0]);
    } else if tic_tac[0][2] != 0 && tic_tac[0][2] == tic_tac[1][1] && tic_tac[1][1] == tic_tac[2][0] {
        return Some(tic_tac[0][2]);
    }

    None
}
    

fn main() {

    let mut vec1:Vec<Vec<u32>>= vec![vec![0;3];3];
    let mut x= true;
    let mut o = false;
    let zero:&u32=&0;
    loop{
    println!(" Choose your position:\n1 2 3 \n4 5 6\n7 8 9 ");
    let mut position = String::new();
    io::stdin().read_line(&mut position).unwrap();
    let num_position:u32 = position.trim().parse().unwrap();
    if num_position<1 || num_position>9{
        println!("Wrong option");
        continue
    };
    let row:u32 = (num_position-1)/3;
    let col:u32 = (num_position-1)%3;

    if vec1[row as usize][col as usize] != 0{
        println!("This position is taken take something else:",);
        continue;
    };
  
    if x {
            vec1[row as usize][ col as usize]=1;
             x= false;
             o= true
    }
    else if o {
            vec1[row as usize][ col as usize]=2;
             o=false;
             x=true;
    }
    for row in &vec1{
        for val in row{
            print!("{} ",val)
        }
        println!();
    }
   if let Some(winner) =check_winner(&vec1){
    println!("Game over player:{} WON!",winner);
    break;
   }
    
    
}





//
      

}
//1 2 3
//4 5 6
//7 8 9