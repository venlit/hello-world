use std::collections::HashMap;
fn median(v: &mut [i32]) -> f32{
    v.sort();
    if v.len() % 2 == 0{
        return (v[(v.len() / 2) as usize] + v[(v.len() / 2 - 1) as usize]) as f32 / 2.0 
    }
    v[(v.len() / 2) as usize] as f32
}

fn mean(v: &[i32]) -> f32{
    let mut x = 0;
    for i in v.iter(){
       x += i; 
    }
    x as f32 / v.len() as f32 
}

fn mode(v: &[i32]) -> Option<i32>{
    let mut counts = HashMap::new();

    v.iter().copied().max_by_key(|&n| {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
        *count
    })
}

pub fn run(){
    let mut nums = [19, 5, 12, 5, 123, 61, 43 ,15, 69, 420];
    println!("median: {} mean: {} mode: {:?}", median(&mut nums), mean(&nums), mode(&nums));
    nums.sort();
    println!("{:?}", nums);
}