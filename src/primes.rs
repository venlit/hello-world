fn prime(n: u64) -> u64{
    let mut counter = 1;

    for i in (3..n).step_by(2){
        let mut prime = true;

        for j in (3..i/2).step_by(2){
            if i % j == 0{
                prime = false;
                break;
            }
        }

        if prime == true{
            counter += 1;
        }
    }
    counter
}

pub fn run(){
    println!("{}", prime(1000000));
}

