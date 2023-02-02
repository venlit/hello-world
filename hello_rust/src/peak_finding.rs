use std::cmp::Ordering;
fn binary_search(arr: &[i32]) -> Option<i32>{
    let mut max: usize = arr.len() - 1;
    let mut min: usize = 0;
    let mut mid: usize = (max + min) / 2;

    while max > min{

        mid = (max+min) / 2; 

        if arr[mid].cmp(&arr[mid - 1]) == Ordering::Less{
            max = mid;      
        } else if arr[mid].cmp(&arr[mid + 1]) == Ordering::Less{
            min = mid + 1;
        } else {
            return Some(arr[mid]);
        }

    } 
    None
}

pub fn run(){
    let nums = [1, 3, 5, 6, 7, 8, 9, 10, 8, 7, 4];
    let nums2 = [1, 3, 7, 9, 8, 7, 6, 5, 5, 4, 4];
    let n = [1, 3, 4, 5, 6, 6,6,6,];

    println!("{:?}", binary_search(&nums));
    println!("{:?}", binary_search(&nums2));
    println!("{:?}", binary_search(&n));
}
