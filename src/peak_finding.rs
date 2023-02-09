use std::cmp::Ordering;

fn binary_search(arr: &[i32]) -> (Option<i32>, usize){
    let mut max: usize = arr.len() - 1;
    let mut min: usize = 0;

    while max >= min{

        let mid: usize = (max + min) / 2;

        if mid == 0 || mid == arr.len() - 1{
            return (Some(arr[mid]), mid);
        }

        if arr[mid].cmp(&arr[mid - 1]) == Ordering::Less{
            max = mid;      
        } else if arr[mid].cmp(&arr[mid + 1]) == Ordering::Less{
            min = mid + 1;
        } else {
            return (Some(arr[mid]), mid);
        }

    } 
    (None, 0)
}

pub fn run(){
    // let n = [-10005, -10004, -96, -63, -43, -43, -16, -10, -7, -5, -5, -4, -3, -2, -1, 0, 1, 2, 3, 5, 7, 8, 9, 16, 19, 24, 28, 32, 35, 54, 75, 76, 119, 143, 169, 225, 264, 290, 501, 998, 999, 998];
    let n2 = [998, 999, 998, 501, 290, 264, 225, 169, 143, 119, 76, 75, 54, 35, 32, 28, 24, 19, 16, 9, 8, 7, 5, 3, 2, 1, 0, -1, -2, -3, -4, -5, -5, -7, -10, -16, -43, -43, -63, -96, -10004, -10005];

    // println!("{:?}", binary_search(&n));
    println!("{:?}", binary_search(&n2));
}
