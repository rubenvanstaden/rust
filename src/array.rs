fn two_pointer_technique(nums: &[i32], target: i32) -> Option<(usize, usize)> {

    let mut i = 0;
    let mut j = nums.len() - 1;

    while i < j {
        let sum = nums[i] + nums[j];
        if sum == target {
            return Some((i, j));
        } else if sum < target {
            i += 1;
        } else {
            j -= 1;
        }
    }

    None
}

fn max_sum_subarray(nums: &[i32], k: usize) -> Option<i32> {
    if nums.len() < k {
        return None;
    }

    let mut max_sum = 0;
    let mut current_sum = nums[..k].iter().sum();

    for i in k..nums.len() {
        current_sum += nums[i] - nums[i - k];
        max_sum = max_sum.max(current_sum);
    }

    Some(max_sum)
}

fn prefix_sum(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut prefix = vec![0; n + 1];

    for i in 1..=n {
        prefix[i] = prefix[i - 1] + nums[i - 1];
    }

    prefix
}

fn sum_subarray(prefix: &[i32], i: usize, j: usize) -> i32 {
    prefix[j + 1] - prefix[i]
}

fn main() {

    // Two pointer

    let nums = vec![2, 7, 11, 15];
    let target = 9;

    match two_pointer_technique(&nums, target) {
        Some((i, j)) => println!("Found pair at indices {} and {}", i, j),
        None => println!("No pair found"),
    }

    // Sliding Window

    let nums = vec![1, 2, 3, 4, 5];
    let k = 3;

    match max_sum_subarray(&nums, k) {
        Some(max_sum) => println!("Maximum sum of subarray of length {} is {}", k, max_sum),
        None => println!("Input array is too short"),
    }

    // Prefix sum

    let nums = vec![1, 2, 3, 4, 5];
    let prefix = prefix_sum(&nums);

    println!("Prefix sum: {:?}", prefix);

    let i = 1;
    let j = 3;
    let sum = sum_subarray(&prefix, i, j);

    println!("Sum of subarray [{}, {}]: {}", i, j, sum);
}

