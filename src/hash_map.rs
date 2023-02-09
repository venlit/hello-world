use std::collections::HashMap;

fn mode(x: &[i32]) -> i32{
    let mut map = HashMap::new();

    for &i in x{
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let y = map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers");
    
    y
}

pub fn run(){
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let score = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    // scores.entry(String::from("Green")).or_insert(50);
    // println!("{:?}", scores);

    let x = [10, 3, 54, 12, 3, 5, 5, 12, 16];

    println!("{:?}", mode(&x));


}

