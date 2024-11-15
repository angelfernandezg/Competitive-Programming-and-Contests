// returns leaders in array all elements which are greater or equal to every element on it's right or after him

pub fn arrayleader<T>(list: &[T]) -> Vec<T>
where
    T: Ord + Clone,
{
    let mut leaders: Vec<T> = Vec::with_capacity(list.len());
    let mut max = list.last().expect("Empty Array");

    // reverse list, scan and add elements which are greater or equal to the
    // current max
    for i in list.iter().rev() {
        if *i >= *max {
            max = i;
            leaders.push((*max).clone());
        }
    }

    leaders.reverse();
    leaders
}

// runs in O(n) time, in this implementation 3*n in w.c. 
// requires O(n) space 