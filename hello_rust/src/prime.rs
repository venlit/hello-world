fn prime(x: u32) -> u32{
    let mut count = 0;
    let mut iterate = 2;
    let mut prime = true;
    for i in 0..x{
       for j in 2..iterate{
            if iterate % j == 0 && iterate != 2{
                prime = false;
            }
       } 
       if prime == true{
            count += 1;
       }
       iterate += 1;
       prime = true;
    }
    count
}

pub fn run(){
    println!("{} {} {}", prime(13), prime(20), prime(30));
}