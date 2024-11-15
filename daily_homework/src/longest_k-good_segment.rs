// C is largest possible number

pub fn longest_k_run(list: &[usize], k: usize) -> (usize, usize) {
    let mut current_rep = 0;
    let mut current_max = 0;

    // arbitrary large number as capacity
    let mut accumulator: Vec<usize> = Vec::with_capacity(1000005);
    (0..accumulator.capacity()).for_each(|_| accumulator.push(0));

    let mut left_index = 1;
    let mut right_index = 1;

    let mut i: usize = 1;
    let mut j: usize = 1;

    while j <= list.len() {
        accumulator[list[j - 1]] += 1;
        // if number is seen for a first time, add one to the current counter
        // of distinct numbers
        if accumulator[list[j - 1]] == 1 {
            current_rep += 1;
        }

        // move left "pointer" until the segment becomes again "k-good"
        while i <= list.len() && current_rep > k {
            accumulator[list[i - 1]] -= 1;
            if accumulator[list[i - 1]] == 0 {
                current_rep -= 1;
            }
            i += 1;
        }

        // update current_max, left_index and right_index
        if j - i > current_max {
            current_max = j - i;
            left_index = i;
            right_index = j;
        }
        j += 1;
    }

    (left_index, right_index)
}

// runs in O(n), space is O(C)