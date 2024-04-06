fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None; 
    }
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort(); 

    Some(sorted_arr[k - 1])
}

fn main() {
    let arr = [4, 2, 7, 1, 9, 5];
    let k = 3; 

    if let Some(element) = kth_smallest(&arr, k) {
        println!("The {}th smallest element is: {}",k,element);
    } else {
        println!("not found");
    }
}
