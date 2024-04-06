fn find_median(arr: &[i32]) -> Option<f64> {
    if arr.is_empty() {
      return None;
    }
  
    let len = arr.len();
    let mid_index = len / 2;
  
    // Handle even and odd length arrays separately for median calculation
    if len % 2 == 0 {
      let median = (arr[mid_index - 1] as f64 + arr[mid_index] as f64) / 2.0;
      return Some(median);
    } else {
      return Some(arr[mid_index] as f64);
    }
  }
  
  fn main() {
    let numbers = vec![1, 2, 3, 4, 5,7,9,11];
    let median = find_median(&numbers);
  
    if let Some(value) = median {
      println!("The median of the array is: {}", value);
    } else {
      println!("The array is empty");
    }
  }
  