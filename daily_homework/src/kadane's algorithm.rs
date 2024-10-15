// returns the maximum sum of all subarrays
// uses the default value for T as the "zero"
// uses two counters for keeping track of the current subarray sum, which can never be less than "zero", and the max subarray sum

pub fn max_sub_array<T>(list: &[T]) -> T
where
    T: Copy + Ord + Default + AddAssign,
{
    match list {
        [] => return Default::default(),
        [i] => return *i,
        _ => {}
    }

    let mut sum: T = list[0];
    let mut max_sum: T = sum;

    for &i in list[1..list.len()].iter() {
        if sum > Default::default() {
            sum += i;
        } else {
            sum = i;
        }

        if sum > max_sum {
            max_sum = sum;
        }
    }

    max_sum
}

// O(n) time, O(1) space