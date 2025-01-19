use std::io::{self, Read};
fn main() {

    let mut vec1:Vec<Vec<u32>>= vec![vec![0;3];3];
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
    let mut x= true;
    let mut o = false;
    
    if x ==true{
            vec1[row as usize][ col as usize]=1;
             x= false;
             o= true
    }
    else if o ==true{
            vec1[row as usize][ col as usize]=0;
             o=false;
             x=true;
    }
    
}





//
      

}
//1 2 3
//4 5 6
//7 8 9