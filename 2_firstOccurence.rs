fn first_occurrence_index(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut result = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            result = Some(mid);
            high = mid - 1;
        } else if arr[mid] < target {
            
            low = mid + 1;
        } else {
            
            high = mid - 1;
        }
    }
    result
}

fn main() {
    let arr = [1, 2, 2, 2, 3, 4, 5, 5, 6, 6, 7,22];
    let target = 22;

    if let Some(index) = first_occurrence_index(&arr, target) {
        println!("Found at index: {}",index);
    } else {
        println!("not found");
    }
}
