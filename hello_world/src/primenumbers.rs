pub fn run(){
    let mut nums: Vec<i32> = vec![];
    let num: i32 = std::i16::MAX as i32;
    if checkprime(num) == true{
        println!("the number {num} is prime");
    } else{

        println!("the number {num} is not prime");

        for x in num..std::i32::MAX{
            if checkprime(x) == true{
                nums.push(x);
                break;
            }
        }
        for y in (std::i32::MIN..num).rev(){
            if checkprime(y) == true{
                nums.push(y);
                break;
            }  
        }

        if nums[0] - num < num - nums[1]{
            println!("The closest prime number is {}", nums[0]);
        } else if num - nums[1] < nums[0] - num{
            println!("The closest prime number is {}", nums[1]);
        } else{
            println!("The closest prime numbers are {x} and {y}.", x = nums[0], y = nums[1]);
        }
        
    }

    println!("The preceding prime numbers are:");
    for x in 2..num{
        if checkprime(x) == true{
            println!("{x}");
        }
    }

}
fn checkprime(x: i32) -> bool{
    let mut check: bool = true;
    for number in 2..x{
        if x % number == 0{ check = false }
    }
    check
}