fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut current_sum = i32::MIN;
    let mut max_sum = i32::MIN;

    for &element in arr.iter() {
      current_sum = current_sum.max(element); 
      max_sum = max_sum.max(current_sum);  
      current_sum += element; 
    }
  
    return max_sum;
  }
  
  fn main() {
    let numbers = vec![1, -2, 3, 4, -1, 2, 1, 5, -3];
    let max_sum = max_subarray_sum(&numbers);
    println!("Maximum subarray sum: {}", max_sum);
  }
  