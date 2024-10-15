pub fn sliding_window<T>(nums: &[T], k: usize) -> Vec<T>
where
    T: Ord + Copy,
{
    // maximums is the output vector
    let mut maximums = Vec::with_capacity(nums.len() - k + 1);
    // b holds the candidates for the maximum
    let mut b = VecDeque::with_capacity(k);

    for (pos, &v) in nums.iter().enumerate() {
        // remove from the front elements which are not in the window
        // (position is outside)
        while let Some(&(_, front)) = b.front() {
            if front + k > pos {
                break;
            }
            b.pop_front();
        }
        // remove from the back elements which are less than the new inserted
        // value
        while let Some(&(back, _)) = b.back() {
            if back >= v {
                break;
            }
            b.pop_back();
        }
        // add new value
        b.push_back((v, pos));

        // push new maximum to the output list, which is the first element of
        // the queue
        if pos >= k - 1 {
            maximums.push(b.front().unwrap().0);
        }
    }

    maximums
}

// O(n) time and O(n) space