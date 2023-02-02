fn word_extract(str: &String, num: usize) -> &str{
    let bytes = str.as_bytes();
    let mut space: usize = 0;
    let mut tracker: usize = 0;

    for (i, &item) in bytes.iter().enumerate(){

        if item == b' ' || i == str.len() - 1{
            space += 1;

            if space == num && i == str.len() - 1{                
                return &str[tracker..];  
            }else if space == num{
                return &str[tracker..i];
            }

            tracker = i + 1;
        }     
    }
    &str[..]
}

pub fn run(){
    let x = String::from("lost in paradise night and day");
    println!("{}", word_extract(&x, 4));
}