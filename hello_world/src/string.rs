pub fn run(){
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    let s4 = format!("{s1}-{s2}-{s3}");
    for i in s4.chars(){
        println!("{:?}", i);
    }
}